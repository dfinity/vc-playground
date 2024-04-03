use candid::{candid_method, CandidType, Deserialize, Principal};
use canister_sig_util::signature_map::{SignatureMap, LABEL_SIG};
use canister_sig_util::{extract_raw_root_pk_from_der, CanisterSigPublicKey, IC_ROOT_PK_DER};
use ic_cdk::api::{caller, set_certified_data, time};
use ic_cdk_macros::{init, query, update};
use ic_certification::{fork_hash, labeled_hash, pruned, Hash};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::{Bound, Storable};
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableCell};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use meta_issuer::groups_api::{
    AddGroupRequest, FullGroupData, GetGroupRequest, GroupStats, GroupsError, JoinGroupRequest,
    ListGroupsRequest, MemberData, MembershipStatus, PublicGroupData, PublicGroupsData,
    SetUserRequest, UpdateMembershipRequest, UserData,
};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::RefCell;
use vc_util::issuer_api::{
    ArgumentValue, CredentialSpec, DerivationOriginData, DerivationOriginError,
    DerivationOriginRequest, GetCredentialRequest, Icrc21ConsentInfo, Icrc21Error, Icrc21ErrorInfo,
    Icrc21VcConsentMessageRequest, IssueCredentialError, IssuedCredentialData,
    PrepareCredentialRequest, PreparedCredentialData, SignedIdAlias,
};
use vc_util::{
    build_credential_jwt, did_for_principal, get_verified_id_alias_from_jws, vc_jwt_to_jws,
    vc_signing_input, vc_signing_input_hash, AliasTuple, CredentialParams,
};

use asset_util::{collect_assets, CertifiedAssets};
use ic_cdk_macros::post_upgrade;
use std::collections::BTreeMap;

/// We use restricted memory in order to ensure the separation between non-managed config memory (first page)
/// and the managed memory for potential other data of the canister.
type Memory = RestrictedMemory<DefaultMemoryImpl>;
type ConfigCell = StableCell<IssuerConfig, Memory>;
type GroupsMap = StableBTreeMap<String, GroupRecord, VirtualMemory<Memory>>;
type UsersMap = StableBTreeMap<Principal, UserRecord, VirtualMemory<Memory>>;

const GROUPS_MEMORY_ID: MemoryId = MemoryId::new(0u8);
const USERS_MEMORY_ID: MemoryId = MemoryId::new(1u8);

const ISSUER_URL: &str = "https://vc-playground.vc";
const CREDENTIAL_URL_PREFIX: &str = "data:text/plain;charset=UTF-8,";

const MINUTE_NS: u64 = 60 * 1_000_000_000;
const PROD_II_CANISTER_ID: &str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
// The expiration of issued verifiable credentials.
const VC_EXPIRATION_PERIOD_NS: u64 = 15 * MINUTE_NS;

// Internal container of per-group data.
#[derive(CandidType, Clone, Deserialize)]
struct GroupRecord {
    pub created_timestamp_ns: u64,
    pub owner: Principal,
    pub members: BTreeMap<Principal, MemberRecord>,
}

#[derive(CandidType, Clone, Deserialize)]
struct MemberRecord {
    joined_timestamp_ns: u64,
    membership_status: MembershipStatus,
}

#[derive(CandidType, Clone, Deserialize)]
struct UserRecord {
    user_nickname: Option<String>,
    issuer_nickname: Option<String>,
}

impl Storable for UserRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode GroupRecord"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode GroupRecord")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for GroupRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode GroupRecord"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode GroupRecord")
    }
    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    /// Stable structures
    // Static configuration of the canister set by init() or post_upgrade().
    static CONFIG: RefCell<ConfigCell> = RefCell::new(ConfigCell::init(config_memory(), IssuerConfig::default()).expect("failed to initialize stable cell"));

    static MEMORY_MANAGER: RefCell<MemoryManager<Memory>> =
        RefCell::new(MemoryManager::init(managed_memory()));
    static GROUPS : RefCell<GroupsMap> = RefCell::new(
      StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(GROUPS_MEMORY_ID)),
    ));
    static USERS : RefCell<UsersMap> = RefCell::new(
      StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID)),
    ));

    /// Non-stable structures
    // Canister signatures
    static SIGNATURES : RefCell<SignatureMap> = RefCell::new(SignatureMap::default());
    // Assets for the management app
    static ASSETS: RefCell<CertifiedAssets> = RefCell::new(CertifiedAssets::default());
}

lazy_static! {
    // Seed and public key used for signing the credentials.
    static ref CANISTER_SIG_SEED: Vec<u8> = hash_bytes("MetaIssuer").to_vec();
    static ref CANISTER_SIG_PK: CanisterSigPublicKey = CanisterSigPublicKey::new(ic_cdk::id(), CANISTER_SIG_SEED.clone());
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
struct IssuerConfig {
    /// Root of trust for checking canister signatures.
    ic_root_key_raw: Vec<u8>,
    /// List of canister ids that are allowed to provide id alias credentials.
    idp_canister_ids: Vec<Principal>,
    /// The derivation origin to be used by the issuer.
    derivation_origin: String,
    /// Frontend hostname to be used by the issuer.
    frontend_hostname: String,
}

impl Storable for IssuerConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode IssuerConfig"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode IssuerConfig")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Default for IssuerConfig {
    fn default() -> Self {
        let derivation_origin = format!("https://{}.icp0.io", ic_cdk::id().to_text());
        Self {
            ic_root_key_raw: extract_raw_root_pk_from_der(IC_ROOT_PK_DER)
                .expect("failed to extract raw root pk from der"),
            idp_canister_ids: vec![Principal::from_text(PROD_II_CANISTER_ID).unwrap()],
            derivation_origin: derivation_origin.clone(),
            frontend_hostname: derivation_origin, // by default, use DERIVATION_ORIGIN as frontend-hostname
        }
    }
}

impl From<IssuerInit> for IssuerConfig {
    fn from(init: IssuerInit) -> Self {
        Self {
            ic_root_key_raw: extract_raw_root_pk_from_der(&init.ic_root_key_der)
                .expect("failed to extract raw root pk from der"),
            idp_canister_ids: init.idp_canister_ids,
            derivation_origin: init.derivation_origin,
            frontend_hostname: init.frontend_hostname,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct IssuerInit {
    /// Root of trust for checking canister signatures.
    ic_root_key_der: Vec<u8>,
    /// List of canister ids that are allowed to provide id alias credentials.
    idp_canister_ids: Vec<Principal>,
    /// The derivation origin to be used by the issuer.
    derivation_origin: String,
    /// Frontend hostname to be used by the issuer.
    frontend_hostname: String,
}

fn check_authenticated() -> Result<(), GroupsError> {
    if caller() == Principal::anonymous() {
        Err(GroupsError::NotAuthenticated(
            "anonymous caller not permitted".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[init]
#[candid_method(init)]
fn init(init_arg: Option<IssuerInit>) {
    if let Some(init) = init_arg {
        apply_config(init);
    };

    init_assets();
}

#[post_upgrade]
fn post_upgrade(init_arg: Option<IssuerInit>) {
    init(init_arg);
}

/// API for setting/getting user data.
#[query]
#[candid_method(query)]
fn get_user() -> Result<UserData, GroupsError> {
    check_authenticated()?;
    USERS.with_borrow(|users| {
        if let Some(user_record) = users.get(&caller()) {
            Ok(UserData {
                user_nickname: user_record.user_nickname,
                issuer_nickname: user_record.issuer_nickname,
            })
        } else {
            Err(GroupsError::NotFound(format!(
                "user principal: {}",
                caller()
            )))
        }
    })
}

#[update]
#[candid_method]
fn set_user(req: SetUserRequest) -> Result<(), GroupsError> {
    check_authenticated()?;
    USERS.with_borrow_mut(|users| {
        users.insert(
            caller(),
            UserRecord {
                user_nickname: req.user_data.user_nickname,
                issuer_nickname: req.user_data.issuer_nickname,
            },
        );
        Ok(())
    })
}

/// API for obtaining information about groups and group membership.
#[query]
#[candid_method(query)]
fn list_groups(req: ListGroupsRequest) -> Result<PublicGroupsData, GroupsError> {
    let anonymous = caller() == Principal::anonymous();
    GROUPS.with_borrow(|groups| {
        let mut list = vec![];
        for (name, record) in groups.iter() {
            if let Some(substring) = &req.group_name_substring {
                if !name.contains(substring) {
                    break;
                }
            }
            let is_owner = if anonymous {
                None
            } else {
                Some(record.owner == caller())
            };
            let membership_status = if anonymous {
                None
            } else {
                record
                    .members
                    .get(&caller())
                    .map(|member_record| member_record.membership_status.clone())
            };
            list.push(PublicGroupData {
                group_name: name,
                stats: GroupStats {
                    member_count: record.members.len() as u32,
                    created_timestamp_ns: record.created_timestamp_ns,
                },
                is_owner,
                membership_status,
            })
        }
        Ok(PublicGroupsData { groups: list })
    })
}

fn maybe_user_nickname(user: &Principal) -> Option<String> {
    USERS.with_borrow(|users| {
        if let Some(user_record) = users.get(user) {
            user_record.user_nickname
        } else {
            None
        }
    })
}

#[query]
#[candid_method(query)]
fn get_group(req: GetGroupRequest) -> Result<FullGroupData, GroupsError> {
    GROUPS.with_borrow(|groups| {
        if let Some(group_record) = groups.get(&req.group_name) {
            let members: Vec<MemberData> = group_record
                .members
                .iter()
                .map(|(member, record)| MemberData {
                    member: *member,
                    nickname: maybe_user_nickname(member).unwrap_or("".to_string()),
                    joined_timestamp_ns: record.joined_timestamp_ns,
                    membership_status: record.membership_status.clone(),
                })
                .collect();
            Ok(FullGroupData {
                group_name: req.group_name,
                stats: GroupStats {
                    member_count: group_record.members.len() as u32,
                    created_timestamp_ns: group_record.created_timestamp_ns,
                },
                members,
            })
        } else {
            Err(GroupsError::NotFound(format!("group: {}", req.group_name)))
        }
    })
}

#[update]
#[candid_method]
fn add_group(req: AddGroupRequest) -> Result<FullGroupData, GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if groups.get(&req.group_name).is_some() {
            Err(GroupsError::AlreadyExists(format!(
                "group: {}",
                req.group_name
            )))
        } else {
            let created_timestamp_ns = time();
            let previous = groups.insert(
                req.group_name.clone(),
                GroupRecord {
                    created_timestamp_ns,
                    owner: caller(),
                    members: BTreeMap::new(),
                },
            );
            assert!(previous.is_none());
            Ok(FullGroupData {
                group_name: req.group_name,
                stats: GroupStats {
                    member_count: 0,
                    created_timestamp_ns,
                },
                members: vec![],
            })
        }
    })
}

#[update]
#[candid_method]
fn join_group(req: JoinGroupRequest) -> Result<(), GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if let Some(mut group_record) = groups.get(&req.group_name) {
            if let Some(member_record) = group_record.members.get(&caller()) {
                // If a record exists and has `Rejected`-status,
                // switch to `PendingReview` and update timestamp, otherwise do nothing.
                if member_record.membership_status == MembershipStatus::Rejected {
                    group_record.members.insert(
                        caller(),
                        MemberRecord {
                            joined_timestamp_ns: time(),
                            membership_status: MembershipStatus::PendingReview,
                        },
                    );
                }
            } else {
                group_record.members.insert(
                    caller(),
                    MemberRecord {
                        joined_timestamp_ns: time(),
                        membership_status: MembershipStatus::PendingReview,
                    },
                );
            }
            groups.insert(req.group_name, group_record);
            Ok(())
        } else {
            Err(GroupsError::NotFound(format!("group: {}", req.group_name)))
        }
    })
}

#[update]
#[candid_method]
fn update_membership(req: UpdateMembershipRequest) -> Result<(), GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if let Some(mut group_record) = groups.get(&req.group_name) {
            if group_record.owner != caller() {
                return Err(GroupsError::NotAuthorized(format!(
                    "{} is not an owner of {}",
                    caller(),
                    req.group_name
                )));
            }
            for update in req.updates {
                if let Some(member_record) = group_record.members.get(&update.member) {
                    group_record.members.insert(
                        update.member,
                        MemberRecord {
                            joined_timestamp_ns: member_record.joined_timestamp_ns,
                            membership_status: update.new_status,
                        },
                    );
                } else {
                    return Err(GroupsError::NotFound(format!("member: {}", update.member)));
                }
            }
            groups.insert(req.group_name, group_record);
            Ok(())
        } else {
            Err(GroupsError::NotFound(format!("group: {}", req.group_name)))
        }
    })
}

// TODO: restrict or remove `configure()`.
#[update]
#[candid_method]
fn configure(config: IssuerInit) {
    apply_config(config);
}

fn apply_config(init: IssuerInit) {
    CONFIG
        .with_borrow_mut(|config_cell| config_cell.set(IssuerConfig::from(init)))
        .expect("failed to apply issuer config");
}

fn authorize_vc_request(
    alias: &SignedIdAlias,
    expected_vc_subject: &Principal,
    current_time_ns: u128,
) -> Result<AliasTuple, IssueCredentialError> {
    CONFIG.with_borrow(|config| {
        let config = config.get();

        for idp_canister_id in &config.idp_canister_ids {
            if let Ok(alias_tuple) = get_verified_id_alias_from_jws(
                &alias.credential_jws,
                expected_vc_subject,
                idp_canister_id,
                &config.ic_root_key_raw,
                current_time_ns,
            ) {
                return Ok(alias_tuple);
            }
        }
        Err(IssueCredentialError::InvalidIdAlias(
            "id alias could not be verified".to_string(),
        ))
    })
}

#[update]
#[candid_method]
async fn prepare_credential(
    req: PrepareCredentialRequest,
) -> Result<PreparedCredentialData, IssueCredentialError> {
    let alias_tuple = match authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
        Ok(alias_tuple) => alias_tuple,
        Err(err) => return Err(err),
    };

    let credential_jwt = match prepare_credential_jwt(&req.credential_spec, &alias_tuple) {
        Ok(credential) => credential,
        Err(err) => return Result::<PreparedCredentialData, IssueCredentialError>::Err(err),
    };
    let signing_input =
        vc_signing_input(&credential_jwt, &CANISTER_SIG_PK).expect("failed getting signing_input");
    let msg_hash = vc_signing_input_hash(&signing_input);

    SIGNATURES.with(|sigs| {
        let mut sigs = sigs.borrow_mut();
        sigs.add_signature(&CANISTER_SIG_SEED, msg_hash);
    });
    update_root_hash();
    Ok(PreparedCredentialData {
        prepared_context: Some(ByteBuf::from(credential_jwt.as_bytes())),
    })
}

fn update_root_hash() {
    SIGNATURES.with_borrow(|sigs| {
        ASSETS.with_borrow(|assets| {
            let prefixed_root_hash = fork_hash(
                // NB: Labels added in lexicographic order.
                &assets.root_hash(),
                &labeled_hash(LABEL_SIG, &sigs.root_hash()),
            );

            set_certified_data(&prefixed_root_hash[..]);
        })
    })
}

#[query]
#[candid_method(query)]
fn get_credential(req: GetCredentialRequest) -> Result<IssuedCredentialData, IssueCredentialError> {
    if let Err(err) = authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
        return Result::<IssuedCredentialData, IssueCredentialError>::Err(err);
    };
    let prepared_context = match req.prepared_context {
        Some(context) => context,
        None => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(internal_error(
                "missing prepared_context",
            ))
        }
    };
    let credential_jwt = match String::from_utf8(prepared_context.into_vec()) {
        Ok(s) => s,
        Err(_) => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(internal_error(
                "invalid prepared_context",
            ))
        }
    };
    let signing_input =
        vc_signing_input(&credential_jwt, &CANISTER_SIG_PK).expect("failed getting signing_input");
    let message_hash = vc_signing_input_hash(&signing_input);
    let sig_result = SIGNATURES.with(|sigs| {
        let sig_map = sigs.borrow();
        let certified_assets_root_hash = ASSETS.with_borrow(|assets| assets.root_hash());
        sig_map.get_signature_as_cbor(
            &CANISTER_SIG_SEED,
            message_hash,
            Some(certified_assets_root_hash),
        )
    });
    let sig = match sig_result {
        Ok(sig) => sig,
        Err(e) => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(
                IssueCredentialError::SignatureNotFound(format!(
                    "signature not prepared or expired: {}",
                    e
                )),
            );
        }
    };
    let vc_jws =
        vc_jwt_to_jws(&credential_jwt, &CANISTER_SIG_PK, &sig).expect("failed constructing JWS");
    Result::<IssuedCredentialData, IssueCredentialError>::Ok(IssuedCredentialData { vc_jws })
}

#[update]
#[candid_method]
async fn vc_consent_message(
    req: Icrc21VcConsentMessageRequest,
) -> Result<Icrc21ConsentInfo, Icrc21Error> {
    get_vc_consent_message_en(&req.credential_spec)
}

#[update]
#[candid_method]
async fn derivation_origin(
    req: DerivationOriginRequest,
) -> Result<DerivationOriginData, DerivationOriginError> {
    get_derivation_origin(&req.frontend_hostname)
}

fn get_derivation_origin(hostname: &str) -> Result<DerivationOriginData, DerivationOriginError> {
    CONFIG.with_borrow(|config| {
        let config = config.get();
        if hostname == config.frontend_hostname {
            Ok(DerivationOriginData {
                origin: config.derivation_origin.clone(),
            })
        } else {
            Err(DerivationOriginError::UnsupportedOrigin(
                hostname.to_string(),
            ))
        }
    })
}

const VERIFIED_MEMBER_VC_CONSENT_EN: &str = r###"# Verified Member

Credential stating that you are a member of group "###;

pub fn get_vc_consent_message_en(
    credential_spec: &CredentialSpec,
) -> Result<Icrc21ConsentInfo, Icrc21Error> {
    match verify_spec_and_get_group_name(credential_spec) {
        Err(err) => Err(Icrc21Error::ConsentMessageUnavailable(Icrc21ErrorInfo {
            description: err,
        })),
        Ok(group_name) => Ok(Icrc21ConsentInfo {
            consent_message: format!("{} '{}'.", VERIFIED_MEMBER_VC_CONSENT_EN, group_name),
            language: "en".to_string(),
        }),
    }
}

fn verify_spec_and_get_group_name(spec: &CredentialSpec) -> Result<String, String> {
    if spec.credential_type.as_str() == "VerifiedMember" {
        let Some(arguments) = &spec.arguments else {
            return Err("Credential spec has no arguments".to_string());
        };
        let expected_argument = "groupName";
        let Some(value) = arguments.get(expected_argument) else {
            return Err(format!(
                "Credential spec has no {}-argument",
                expected_argument
            ));
        };
        if arguments.len() != 1 {
            return Err("Credential spec has unexpected arguments".to_string());
        }
        let ArgumentValue::String(group_name) = value else {
            return Err(format!(
                "Credential spec has unexpected value for {}-argument",
                expected_argument
            ));
        };
        Ok(group_name.to_string())
    } else {
        Err(format!(
            "Credential {} is not supported",
            spec.credential_type.as_str()
        ))
    }
}

#[query]
#[candid_method(query)]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    // TODO: add `/metrics`-endpoint
    let parts: Vec<&str> = req.url.split('?').collect();
    let path = parts[0];
    let sigs_root_hash =
        SIGNATURES.with_borrow(|sigs| pruned(labeled_hash(LABEL_SIG, &sigs.root_hash())));
    let maybe_asset = ASSETS.with_borrow(|assets| {
        assets.get_certified_asset(path, req.certificate_version, Some(sigs_root_hash))
    });

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

fn verified_member_credential(
    subject_principal: Principal,
    credential_spec: &CredentialSpec,
) -> String {
    let params = CredentialParams {
        spec: credential_spec.clone(),
        subject_id: did_for_principal(subject_principal),
        credential_id_url: credential_id_for_principal(subject_principal),
        issuer_url: ISSUER_URL.to_string(),
        expiration_timestamp_s: exp_timestamp_s(),
    };
    build_credential_jwt(params)
}

fn exp_timestamp_s() -> u32 {
    ((time() + VC_EXPIRATION_PERIOD_NS) / 1_000_000_000) as u32
}

// Prepares a unique id for the given subject_principal.
// The returned URL has the format: "data:text/plain;charset=UTF-8,issuer:...,timestamp_ns:...,subject:..."
fn credential_id_for_principal(subject_principal: Principal) -> String {
    let issuer = format!("issuer:{}", ISSUER_URL);
    let timestamp = format!("timestamp_ns:{}", time());
    let subject = format!("subject:{}", subject_principal.to_text());
    format!(
        "{}{},{},{}",
        CREDENTIAL_URL_PREFIX, issuer, timestamp, subject
    )
}

fn prepare_credential_jwt(
    credential_spec: &CredentialSpec,
    alias_tuple: &AliasTuple,
) -> Result<String, IssueCredentialError> {
    let group_name = verify_spec_and_get_group_name(credential_spec)
        .map_err(IssueCredentialError::UnsupportedCredentialSpec)?;
    GROUPS.with_borrow(|groups| {
        verify_principal_is_member(alias_tuple.id_dapp, group_name, groups)
    })?;
    Ok(verified_member_credential(
        alias_tuple.id_alias,
        credential_spec,
    ))
}

fn verify_principal_is_member(
    user: Principal,
    group_name: String,
    groups: &GroupsMap,
) -> Result<(), IssueCredentialError> {
    if let Some(group_record) = groups.get(&group_name) {
        if let Some(member_record) = group_record.members.get(&user) {
            if member_record.membership_status == MembershipStatus::Accepted {
                return Ok(());
            }
        }
    }
    Err(IssueCredentialError::UnauthorizedSubject(
        "not an accepted member".to_string(),
    ))
}

fn internal_error(msg: &str) -> IssueCredentialError {
    IssueCredentialError::Internal(String::from(msg))
}

fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.as_ref());
    hasher.finalize().into()
}

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
pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
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
        println!("***** interface: {}", canister_interface);
        service_equal(
            CandidSource::Text(&canister_interface),
            CandidSource::File(Path::new("meta_issuer.did")),
        )
        .unwrap_or_else(|e| {
            panic!(
                "the canister code interface is not equal to the did file: {:?}",
                e
            )
        });
    }
}
