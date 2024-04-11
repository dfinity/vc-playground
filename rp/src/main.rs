use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::{caller, set_certified_data, time};
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::{Bound, Storable};
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableCell};
use include_dir::{include_dir, Dir};
use relying_party::rp_api::{
    AddExclusiveContentRequest, ContentData, ContentError, ExclusiveContentList, HttpRequest,
    HttpResponse, ImageData, ImagesList, ListExclusiveContentRequest, ListImagesRequest, RpInit,
    UploadImagesRequest, ValidateVpRequest,
};
use serde_bytes::ByteBuf;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

use asset_util::{collect_assets, CertifiedAssets};
use canister_sig_util::extract_raw_root_pk_from_der;
use ic_cdk_macros::post_upgrade;
use vc_util::{validate_ii_presentation_and_claims, VcFlowSigners};

/// We use restricted memory in order to ensure the separation between non-managed config memory (first page)
/// and the managed memory for potential other data of the canister.
type Memory = RestrictedMemory<DefaultMemoryImpl>;
type ConfigCell = StableCell<RpConfig, Memory>;
type ImagesMap = StableBTreeMap<String, ImageRecord, VirtualMemory<Memory>>;
type ExclusiveContentMap = StableBTreeMap<String, ExclusiveContentRecord, VirtualMemory<Memory>>;

const IMAGES_MEMORY_ID: MemoryId = MemoryId::new(0u8);
const EXCLUSIVE_CONTENT_MEMORY_ID: MemoryId = MemoryId::new(1u8);

// Internal container of per-image data.
#[derive(CandidType, Clone, Deserialize)]
struct ImageRecord {
    pub bytes: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
struct ExclusiveContentRecord {
    owner: Principal,
    created_timestamp_ns: u64,
    url: String,
    credential_group_name: String,
    credential_group_owner: Principal,
}

impl Storable for ImageRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode ImageRecord"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode ImageRecord")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for ExclusiveContentRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode ExclusiveContentRecord"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode ExclusiveContentRecord")
    }
    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    /// Stable structures
    // Static configuration of the canister set by init(), configure(), or post_upgrade().
    static CONFIG: RefCell<ConfigCell> = RefCell::new(ConfigCell::init(config_memory(), RpConfig::default()).expect("failed to initialize stable cell"));

    static MEMORY_MANAGER: RefCell<MemoryManager<Memory>> =
        RefCell::new(MemoryManager::init(managed_memory()));
    static IMAGES : RefCell<ImagesMap> = RefCell::new(
      StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(IMAGES_MEMORY_ID)),
    ));

    static EXCLUSIVE_CONTENT : RefCell<ExclusiveContentMap> = RefCell::new(
      StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EXCLUSIVE_CONTENT_MEMORY_ID)),
    ));
    /// Non-stable structures
    // Assets for the management app
    static ASSETS: RefCell<CertifiedAssets> = RefCell::new(CertifiedAssets::default());
}

/// Reserve the first stable memory page for the configuration stable cell.
fn config_memory() -> Memory {
    RestrictedMemory::new(DefaultMemoryImpl::default(), 0..1)
}

/// All the stable memory after the first page is managed by MemoryManager
fn managed_memory() -> Memory {
    RestrictedMemory::new(
        DefaultMemoryImpl::default(),
        1..ic_stable_structures::MAX_PAGES,
    )
}

#[derive(CandidType, Deserialize)]
struct RpConfig {
    /// Root of trust for checking canister signatures.
    ic_root_key_raw: Vec<u8>,

    /// II instance that is allowed to provide id alias credentials.
    ii_origin: String,
    ii_canister_id: Principal,

    /// Issuers that are trusted by this relying party.
    /// (a map from the origin to canister id)
    issuers: BTreeMap<String, Principal>,
}

impl From<RpInit> for RpConfig {
    fn from(init: RpInit) -> Self {
        Self {
            ic_root_key_raw: extract_raw_root_pk_from_der(&init.ic_root_key_der)
                .expect("failed to extract raw root pk from der"),
            ii_origin: init.ii_origin,
            ii_canister_id: init.ii_canister_id,
            issuers: init
                .issuers
                .iter()
                .map(|data| (data.origin.to_string(), data.canister_id))
                .collect(),
        }
    }
}

impl Storable for RpConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode RpConfig"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode RpConfig")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Default for RpConfig {
    fn default() -> Self {
        Self {
            ic_root_key_raw: vec![],
            ii_origin: "".to_string(),
            ii_canister_id: Principal::anonymous(),
            issuers: BTreeMap::new(),
        }
    }
}

#[init]
#[candid_method(init)]
fn init(init_arg: Option<RpInit>) {
    if let Some(init) = init_arg {
        apply_config(init.into());
    };
    init_assets();
    init_images_map();
}

#[post_upgrade]
fn post_upgrade(init_arg: Option<RpInit>) {
    init(init_arg);
}

fn image_name_to_url(image_name: &str) -> String {
    format!("/images/{}", image_name)
}

fn init_images_map() {
    let img_names = [
        "consensus.png",
        "faultTolerance.png",
        "infiniteScalability.png",
        "internetIdentity.png",
        "messageRouting.png",
        "motoko.png",
        "networkNervousSystem.png",
        "peerToPeer.png",
        "protocolUpgrade.png",
        "sdk.png",
        "serviceNervousSystem.png",
    ];
    IMAGES.with_borrow_mut(|images| {
        for img_name in img_names {
            let _ = images.insert(img_name.to_string(), ImageRecord { bytes: vec![] });
        }
    });
}

/// API for obtaining info about images and exclusive content.
#[query]
#[candid_method(query)]
fn list_images(_req: ListImagesRequest) -> Result<ImagesList, ContentError> {
    IMAGES.with_borrow(|images| {
        let mut list = vec![];
        for (image_name, _record) in images.iter() {
            list.push(ImageData {
                url: image_name_to_url(&image_name),
            })
        }
        Ok(ImagesList { images: list })
    })
}

#[query]
#[candid_method(query)]
fn list_exclusive_content(
    _req: ListExclusiveContentRequest,
) -> Result<ExclusiveContentList, ContentError> {
    EXCLUSIVE_CONTENT.with_borrow(|content| {
        let mut list = vec![];
        for (content_name, record) in content.iter() {
            list.push(ContentData {
                content_name,
                owner: record.owner,
                url: record.url,
                created_timestamp_ns: record.created_timestamp_ns,
                credential_group_name: record.credential_group_name,
                credential_group_owner: record.credential_group_owner,
            })
        }
        Ok(ExclusiveContentList {
            content_items: list,
        })
    })
}

#[update]
#[candid_method]
fn add_exclusive_content(req: AddExclusiveContentRequest) -> Result<ContentData, ContentError> {
    EXCLUSIVE_CONTENT.with_borrow_mut(|content| {
        let data = ContentData {
            content_name: req.content_name,
            owner: caller(),
            url: req.url,
            created_timestamp_ns: time(),
            credential_group_name: req.credential_group_name,
            credential_group_owner: req.credential_group_owner,
        };

        content.insert(
            data.content_name.clone(),
            ExclusiveContentRecord {
                owner: data.owner,
                created_timestamp_ns: data.created_timestamp_ns,
                url: data.url.clone(),
                credential_group_name: data.credential_group_name.clone(),
                credential_group_owner: data.credential_group_owner,
            },
        );
        Ok(data)
    })
}

#[update]
#[candid_method]
fn validate_ii_vp(req: ValidateVpRequest) -> Result<(), ContentError> {
    let (ic_root_key_raw, vc_flow_signers) = CONFIG.with_borrow(|config| {
        let config = config.get();
        let Some(issuer_canister_id) = config.issuers.get(&req.issuer_origin) else {
            return Err(ContentError::NotAuthorized(format!(
                "issuer not supported: {}",
                req.issuer_origin
            )));
        };
        if let Some(issuer_canister_id_from_req) = req.issuer_canister_id {
            if *issuer_canister_id != issuer_canister_id_from_req {
                return Err(ContentError::NotAuthorized(format!(
                    "wrong issuer canister id: expected {}, got {}",
                    issuer_canister_id, issuer_canister_id_from_req
                )));
            }
        }
        Ok((
            config.ic_root_key_raw.clone(),
            VcFlowSigners {
                ii_origin: config.ii_origin.clone(),
                ii_canister_id: config.ii_canister_id,
                issuer_origin: req.issuer_origin,
                issuer_canister_id: *issuer_canister_id,
            },
        ))
    })?;
    match validate_ii_presentation_and_claims(
        &req.vp_jwt,
        req.effective_vc_subject,
        &vc_flow_signers,
        &req.credential_spec,
        &ic_root_key_raw,
        time() as u128,
    ) {
        Ok(()) => Ok(()),
        Err(err) => Err(ContentError::NotAuthorized(format!(
            "VP validation error: {:?}",
            err
        ))),
    }
}

// TODO: restrict or remove `configure()`.
#[update]
#[candid_method]
fn configure(init: RpInit) {
    apply_config(init.into());
}

fn apply_config(config: RpConfig) {
    CONFIG
        .with_borrow_mut(|config_cell| config_cell.set(config))
        .expect("failed to apply RP config");
}

#[update]
#[candid_method]
async fn upload_images(_req: UploadImagesRequest) -> Result<ImagesList, ContentError> {
    panic!("Not implemented");
}

fn update_root_hash() {
    ASSETS.with_borrow(|assets| {
        set_certified_data(&assets.root_hash());
    })
}

#[query]
#[candid_method(query)]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();
    let path = parts[0];
    let maybe_asset = ASSETS
        .with_borrow(|assets| assets.get_certified_asset(path, req.certificate_version, None));

    let mut headers = static_headers();
    match maybe_asset {
        Some(asset) => {
            headers.extend(asset.headers);
            HttpResponse {
                status_code: 200,
                body: ByteBuf::from(asset.content),
                headers,
            }
        }
        None => HttpResponse {
            status_code: 404,
            headers,
            body: ByteBuf::from(format!("Asset {} not found.", path)),
        },
    }
}

fn static_headers() -> Vec<(String, String)> {
    vec![("Access-Control-Allow-Origin".to_string(), "*".to_string())]
}

fn main() {}

// Order dependent: do not move above any function annotated with #[candid_method]!
candid::export_service!();

// Assets
static ASSET_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");
pub fn init_assets() {
    ASSETS.with_borrow_mut(|assets| {
        *assets = CertifiedAssets::certify_assets(
            collect_assets(&ASSET_DIR, Some(fixup_html)),
            &static_headers(),
        );
    });
    update_root_hash()
}

fn fixup_html(html: &str) -> String {
    let canister_id = ic_cdk::api::id();

    // the string we are replacing here is part of the astro main Layout
    html.replace(
        r#"data-app"#,
        &format!(r#"data-app data-canister-id="{canister_id}""#).to_string(),
    )
}

#[cfg(test)]
mod test {
    use crate::__export_service;
    use candid_parser::utils::{service_equal, CandidSource};
    use std::path::Path;

    /// Checks candid interface type equality by making sure that the service in the did file is
    /// equal to the generated interface.
    #[test]
    fn check_candid_interface_compatibility() {
        let canister_interface = __export_service();
        service_equal(
            CandidSource::Text(&canister_interface),
            CandidSource::File(Path::new("rp.did")),
        )
        .unwrap_or_else(|e| {
            panic!(
                "the canister code interface is not equal to the did file: {:?}\n code interface:\n{}",
                e, canister_interface
            )
        });
    }
}
