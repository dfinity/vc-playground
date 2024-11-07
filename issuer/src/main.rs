/// An implementation of a meta-issuer for demonstration purposes.
/// See meta_issuer.did for more info about the architecture and conventions.
use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_canister_sig_creation::signature_map::{CanisterSigInputs, SignatureMap, LABEL_SIG};
use ic_canister_sig_creation::{
    extract_raw_root_pk_from_der, CanisterSigPublicKey, IC_ROOT_PK_DER,
};
use ic_cdk::api::{caller, set_certified_data, time};
use ic_cdk_macros::{init, query, update};
use ic_certification::{fork_hash, labeled_hash, pruned, Hash};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::{Bound, Storable};
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableCell};
use ic_verifiable_credentials::issuer_api::{
    ArgumentValue, CredentialSpec, DerivationOriginData, DerivationOriginError,
    DerivationOriginRequest, GetCredentialRequest, Icrc21ConsentInfo, Icrc21Error, Icrc21ErrorInfo,
    Icrc21VcConsentMessageRequest, IssueCredentialError, IssuedCredentialData,
    PrepareCredentialRequest, PreparedCredentialData, SignedIdAlias,
};
use ic_verifiable_credentials::{
    build_credential_jwt, did_for_principal, get_verified_id_alias_from_jws, vc_jwt_to_jws,
    vc_signing_input, AliasTuple, CredentialParams, VC_SIGNING_INPUT_DOMAIN,
};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use meta_issuer::groups_api::{
    AddGroupRequest, ArgumentValue as OrdArgumentValue, CredentialSpec as OrdCredentialSpec,
    FullGroupData, GetGroupRequest, GroupStats, GroupType, GroupTypes, GroupsError,
    JoinGroupRequest, ListGroupsRequest, MemberData, MembershipStatus, PublicGroupData,
    PublicGroupsData, SetUserRequest, UpdateMembershipRequest, UserData, VcArguments,
};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::RefCell;

use asset_util::{collect_assets, CertifiedAssets};
use ic_cdk_macros::post_upgrade;
use std::collections::BTreeMap;

#[cfg(target_arch = "wasm32")]
use ic_cdk::println;

/// We use restricted memory in order to ensure the separation between non-managed config memory (first page)
/// and the managed memory for potential other data of the canister.
type Memory = RestrictedMemory<DefaultMemoryImpl>;
type ConfigCell = StableCell<IssuerConfig, Memory>;
type GroupsMap = StableBTreeMap<GroupKey, GroupRecord, VirtualMemory<Memory>>;
type UsersMap = StableBTreeMap<Principal, UserRecord, VirtualMemory<Memory>>;

const GROUPS_MEMORY_ID: MemoryId = MemoryId::new(0u8);
const USERS_MEMORY_ID: MemoryId = MemoryId::new(1u8);

const ISSUER_URL: &str = "https://metaissuer.vc";
const CREDENTIAL_URL_PREFIX: &str = "data:text/plain;charset=UTF-8,";

const MINUTE_NS: u64 = 60 * 1_000_000_000;
const PROD_II_CANISTER_ID: &str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
// The expiration of issued verifiable credentials.
const VC_EXPIRATION_PERIOD_NS: u64 = 15 * MINUTE_NS;

// VerifiedAge-credentials need special handling.
const VERIFIED_AGE_CREDENTIAL_TYPE: &str = "VerifiedAge";

// Internal container of per-group data.
#[derive(CandidType, Clone, Deserialize)]
struct GroupRecord {
    pub created_timestamp_ns: u64,
    pub members: BTreeMap<Principal, MemberRecord>,
}

// Tuple that identifies any group.  Note that using owner's principal in a real-world
// application would not be advisable, due to potential privacy breach.
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
struct GroupKey {
    group_name: String,
    owner: Principal,
}

#[derive(CandidType, Clone, Deserialize)]
struct MemberRecord {
    joined_timestamp_ns: u64,
    membership_status: MembershipStatus,
    vc_arguments: Option<VcArguments>,
}

#[derive(CandidType, Clone, Deserialize)]
struct UserRecord {
    user_nickname: Option<String>,
    issuer_nickname: Option<String>,
}

impl From<(String, Principal)> for GroupKey {
    fn from(value: (String, Principal)) -> Self {
        Self {
            group_name: value.0,
            owner: value.1,
        }
    }
}

impl Storable for GroupKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("failed to encode GroupRecord"))
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("failed to decode GroupRecord")
    }
    const BOUND: Bound = Bound::Unbounded;
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

    // Supported group types/credential types.
    static ref GROUP_TYPES: Vec<GroupType> = vec![
            GroupType {
                group_name: "Verified Residence".to_string(),
                credential_spec: OrdCredentialSpec {
                    credential_type: "VerifiedResidence".to_string(),
                    arguments: Some(
                        [(
                            "countryName".to_string(),
                            OrdArgumentValue::String("<country>".to_string()),
                        )]
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    ),
                },
            },
            GroupType {
                group_name: "Verified Age".to_string(),
                credential_spec: OrdCredentialSpec {
                    credential_type: VERIFIED_AGE_CREDENTIAL_TYPE.to_string(),
                    arguments: Some(
                        [("ageAtLeast".to_string(), OrdArgumentValue::Int(18))]
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                    ),
                },
            },
            GroupType {
                group_name: "Verified Employment".to_string(),
                credential_spec: OrdCredentialSpec {
                    credential_type: "VerifiedEmployment".to_string(),
                    arguments: Some(
                        [(
                            "employerName".to_string(),
                            OrdArgumentValue::String("<employer>".to_string()),
                        )]
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    ),
                },
            },
            GroupType {
                group_name: "Verified Humanity".to_string(),
                credential_spec: OrdCredentialSpec {
                    credential_type: "VerifiedHumanity".to_string(),
                    arguments: None,
                },
            },
        ];
    static ref GROUP_NAME_FOR_CREDENTIAL_TYPE: BTreeMap<String, String> = {
        let mut map = BTreeMap::new();
        for group_type in GROUP_TYPES.iter() {
            map.insert(group_type.credential_spec.credential_type.clone(), group_type.group_name.clone());
        }
        map
    };
    static ref CREDENTIAL_SPEC_FOR_GROUP_NAME: BTreeMap<String, OrdCredentialSpec> = {
        let mut map = BTreeMap::new();
        for group_type in GROUP_TYPES.iter() {
            map.insert(group_type.credential_spec.credential_type.clone(), group_type.credential_spec.clone());
        }
        map
    };
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

fn ensure_unique_nicknames(
    new_user_data: &UserData,
    user_principal: Principal,
    users: &UsersMap,
) -> Result<(), GroupsError> {
    if let Some(ref user_nickname) = new_user_data.user_nickname {
        let new_user_nickname = Some(user_nickname.clone());
        for (principal, user_record) in users.iter() {
            if user_record.user_nickname == new_user_nickname && principal != user_principal {
                return Err(GroupsError::AlreadyExists(format!(
                    "user nickname: {}",
                    user_nickname
                )));
            }
        }
    }
    if let Some(ref issuer_nickname) = new_user_data.issuer_nickname {
        let new_issuer_nickname = Some(issuer_nickname.clone());
        for (principal, user_record) in users.iter() {
            if user_record.issuer_nickname == new_issuer_nickname && principal != user_principal {
                return Err(GroupsError::AlreadyExists(format!(
                    "issuer nickname: {}",
                    issuer_nickname
                )));
            }
        }
    }
    Ok(())
}

#[update]
#[candid_method]
fn set_user(req: SetUserRequest) -> Result<(), GroupsError> {
    check_authenticated()?;
    USERS.with_borrow_mut(|users| {
        ensure_unique_nicknames(&req.user_data, caller(), users)?;
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
fn group_types() -> Result<GroupTypes, GroupsError> {
    Ok(GroupTypes {
        types: GROUP_TYPES.clone(),
    })
}

#[query]
#[candid_method(query)]
fn list_groups(req: ListGroupsRequest) -> Result<PublicGroupsData, GroupsError> {
    let anonymous = caller() == Principal::anonymous();
    GROUPS.with_borrow(|groups| {
        let mut list = vec![];
        for (key, group_rec) in groups.iter() {
            if let Some(substring) = &req.group_name_substring {
                if !key.group_name.contains(substring) {
                    break;
                }
            }
            let (membership_status, vc_arguments) = if anonymous {
                (None, None)
            } else {
                group_rec
                    .members
                    .get(&caller())
                    .map_or((None, None), |member_rec| {
                        (
                            Some(member_rec.membership_status.clone()),
                            member_rec.vc_arguments.clone(),
                        )
                    })
            };
            list.push(PublicGroupData {
                group_name: key.group_name,
                owner: key.owner,
                issuer_nickname: maybe_issuer_nickname(&key.owner).unwrap_or("".to_string()),
                stats: GroupStats {
                    member_count: group_rec.members.len() as u32,
                    created_timestamp_ns: group_rec.created_timestamp_ns,
                },
                membership_status,
                vc_arguments,
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

fn maybe_issuer_nickname(user: &Principal) -> Option<String> {
    USERS.with_borrow(|users| {
        if let Some(user_record) = users.get(user) {
            user_record.issuer_nickname
        } else {
            None
        }
    })
}

#[query]
#[candid_method(query)]
fn get_group(req: GetGroupRequest) -> Result<FullGroupData, GroupsError> {
    GROUPS.with_borrow(|groups| {
        if let Some(group_record) = groups.get(&(req.group_name.clone(), caller()).into()) {
            let members: Vec<MemberData> = group_record
                .members
                .iter()
                .map(|(member, member_rec)| MemberData {
                    member: *member,
                    nickname: maybe_user_nickname(member).unwrap_or("".to_string()),
                    joined_timestamp_ns: member_rec.joined_timestamp_ns,
                    membership_status: member_rec.membership_status.clone(),
                    vc_arguments: member_rec.vc_arguments.clone(),
                })
                .collect();
            Ok(FullGroupData {
                group_name: req.group_name,
                owner: caller(),
                issuer_nickname: maybe_issuer_nickname(&caller()).unwrap_or("".to_string()),
                stats: GroupStats {
                    member_count: group_record.members.len() as u32,
                    created_timestamp_ns: group_record.created_timestamp_ns,
                },
                members,
            })
        } else {
            Err(GroupsError::NotFound(format!(
                "group: {}, owner: {}",
                req.group_name,
                caller()
            )))
        }
    })
}

#[update]
#[candid_method]
fn add_group(req: AddGroupRequest) -> Result<FullGroupData, GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if groups
            .get(&(req.group_name.clone(), caller()).into())
            .is_some()
        {
            Err(GroupsError::AlreadyExists(format!(
                "group: {}, owner: {}",
                req.group_name,
                caller()
            )))
        } else {
            let created_timestamp_ns = time();
            let previous = groups.insert(
                (req.group_name.clone(), caller()).into(),
                GroupRecord {
                    created_timestamp_ns,
                    members: BTreeMap::new(),
                },
            );
            assert!(previous.is_none());
            Ok(FullGroupData {
                group_name: req.group_name,
                owner: caller(),
                issuer_nickname: maybe_issuer_nickname(&caller()).unwrap_or("".to_string()),
                stats: GroupStats {
                    member_count: 0,
                    created_timestamp_ns,
                },
                members: vec![],
            })
        }
    })
}

fn verify_vc_arguments_match_spec(
    group_name: &str,
    maybe_args: &Option<VcArguments>,
) -> Result<(), String> {
    let Some(group_spec) = CREDENTIAL_SPEC_FOR_GROUP_NAME.get(group_name) else {
        return Ok(()); // not a supported VC-group, nothing to verify
    };
    let derived_spec: CredentialSpec = OrdCredentialSpec {
        credential_type: group_spec.credential_type.clone(),
        arguments: maybe_args.clone(),
    }
    .into();
    verify_vc_spec(&derived_spec)
}

#[update]
#[candid_method]
fn join_group(req: JoinGroupRequest) -> Result<(), GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if let Some(mut group_record) = groups.get(&(req.group_name.clone(), req.owner).into()) {
            verify_vc_arguments_match_spec(&req.group_name, &req.vc_arguments)
                .map_err(GroupsError::Internal)?;
            if let Some(member_record) = group_record.members.get(&caller()) {
                // If a record exists and has `Rejected`-status,
                // switch to `PendingReview` and update vc_arguments and timestamp, otherwise do nothing.
                if member_record.membership_status == MembershipStatus::Rejected {
                    group_record.members.insert(
                        caller(),
                        MemberRecord {
                            joined_timestamp_ns: time(),
                            membership_status: MembershipStatus::PendingReview,
                            vc_arguments: req.vc_arguments.clone(),
                        },
                    );
                }
            } else {
                group_record.members.insert(
                    caller(),
                    MemberRecord {
                        joined_timestamp_ns: time(),
                        membership_status: MembershipStatus::PendingReview,
                        vc_arguments: req.vc_arguments.clone(),
                    },
                );
            }
            groups.insert((req.group_name, req.owner).into(), group_record);
            Ok(())
        } else {
            Err(GroupsError::NotFound(format!(
                "group: {}, owner: {}",
                req.group_name, req.owner
            )))
        }
    })
}

#[update]
#[candid_method]
fn update_membership(req: UpdateMembershipRequest) -> Result<(), GroupsError> {
    GROUPS.with_borrow_mut(|groups| {
        if let Some(mut group_record) = groups.get(&(req.group_name.clone(), caller()).into()) {
            for update in req.updates {
                if let Some(member_record) = group_record.members.get(&update.member) {
                    group_record.members.insert(
                        update.member,
                        MemberRecord {
                            joined_timestamp_ns: member_record.joined_timestamp_ns,
                            membership_status: update.new_status,
                            vc_arguments: member_record.vc_arguments.clone(),
                        },
                    );
                } else {
                    return Err(GroupsError::NotFound(format!("member: {}", update.member)));
                }
            }
            groups.insert((req.group_name, caller()).into(), group_record);
            Ok(())
        } else {
            Err(GroupsError::NotFound(format!(
                "group: {}, owner: {}",
                req.group_name,
                caller()
            )))
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
            println!(
                "*** checking id_alias for subject {} with IDP {} and derivation origin {}",
                expected_vc_subject, idp_canister_id, config.derivation_origin,
            );
            match get_verified_id_alias_from_jws(
                &alias.credential_jws,
                expected_vc_subject,
                &config.derivation_origin,
                idp_canister_id,
                &config.ic_root_key_raw,
                current_time_ns,
            ) {
                Ok(alias_tuple) => return Ok(alias_tuple),
                Err(err) => {
                    println!("Error checking the id_alias {:?}", err);
                }
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

    SIGNATURES.with(|sigs| {
        let mut sigs = sigs.borrow_mut();
        let sig_inputs = CanisterSigInputs {
            domain: VC_SIGNING_INPUT_DOMAIN,
            seed: CANISTER_SIG_SEED.as_slice(),
            message: signing_input.as_slice(),
        };
        sigs.add_signature(&sig_inputs);
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
    let sig_result = SIGNATURES.with(|sigs| {
        let sig_map = sigs.borrow();
        let certified_assets_root_hash = ASSETS.with_borrow(|assets| assets.root_hash());
        let sig_inputs = CanisterSigInputs {
            domain: VC_SIGNING_INPUT_DOMAIN,
            seed: CANISTER_SIG_SEED.as_slice(),
            message: signing_input.as_slice(),
        };
        sig_map.get_signature_as_cbor(&sig_inputs, Some(certified_assets_root_hash))
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

fn get_derivation_origin(_hostname: &str) -> Result<DerivationOriginData, DerivationOriginError> {
    CONFIG.with_borrow(|config| {
        let config = config.get();
        println!("*** derivation origin: {}", config.derivation_origin);
        Ok(DerivationOriginData {
            origin: config.derivation_origin.clone(),
        })
    })
}

fn args_to_string(spec: &CredentialSpec) -> String {
    let mut args = String::new();
    let Some(arguments) = &spec.arguments else {
        return args;
    };
    for (key, value) in arguments.iter() {
        let arg = format!("{}: {}\n", key, value);
        args.push_str(&arg);
    }
    args
}

pub fn get_vc_consent_message_en(
    credential_spec: &CredentialSpec,
) -> Result<Icrc21ConsentInfo, Icrc21Error> {
    match verify_spec_and_get_group_owner(credential_spec) {
        Err(err) => Err(Icrc21Error::ConsentMessageUnavailable(Icrc21ErrorInfo {
            description: err,
        })),
        Ok((plain_spec, _owner)) => Ok(Icrc21ConsentInfo {
            consent_message: format!(
                "# \"{}\"\n{}",
                plain_spec.credential_type,
                args_to_string(&plain_spec)
            ),
            language: "en".to_string(),
        }),
    }
}

fn get_int_arg_value(arg_name: &str, spec: &CredentialSpec) -> Result<i32, String> {
    let Some(arguments) = &spec.arguments else {
        return Err("Credential spec has no arguments".to_string());
    };
    let Some(arg_value) = arguments.get(arg_name) else {
        return Err(format!("Credential spec has no {}-argument", arg_name));
    };
    if let ArgumentValue::Int(int_value) = arg_value {
        Ok(*int_value)
    } else {
        Err(format!(
            "Credential spec has an unexpected value for {}-argument",
            arg_name
        ))
    }
}

fn get_string_arg_value(arg_name: &str, spec: &CredentialSpec) -> Result<String, String> {
    let Some(arguments) = &spec.arguments else {
        return Err("Credential spec has no arguments".to_string());
    };
    let Some(arg_value) = arguments.get(arg_name) else {
        return Err(format!("Credential spec has no {}-argument", arg_name));
    };
    if let ArgumentValue::String(str_value) = arg_value {
        Ok(str_value.clone())
    } else {
        Err(format!(
            "Credential spec has an unexpected value for {}-argument",
            arg_name
        ))
    }
}

fn check_number_of_args(
    expected_number_of_args: usize,
    spec: &CredentialSpec,
) -> Result<(), String> {
    let Some(arguments) = &spec.arguments else {
        if expected_number_of_args == 0 {
            return Ok(());
        } else {
            return Err(format!(
                "Credential spec has wrong number of arguments, expected {}, got none",
                expected_number_of_args
            ));
        }
    };
    if arguments.len() == expected_number_of_args {
        Ok(())
    } else {
        Err(format!(
            "Credential spec has wrong number of arguments, expected {}, got {}",
            expected_number_of_args,
            arguments.len()
        ))
    }
}

fn get_owner_from_spec(spec: &CredentialSpec) -> Result<(CredentialSpec, Principal), String> {
    let owner = get_string_arg_value("owner", spec)?;
    let mut plain_spec = spec.to_owned();
    plain_spec
        .arguments
        .as_mut()
        .unwrap()
        .remove("owner")
        .unwrap();
    let owner = Principal::from_text(&owner).map_err(|e| format!("bad owner {}: {}", owner, e))?;
    Ok((plain_spec, owner))
}

fn verify_vc_spec(spec: &CredentialSpec) -> Result<(), String> {
    match spec.credential_type.as_str() {
        "VerifiedResidence" => {
            check_number_of_args(1, spec)?;
            let _country_name = get_string_arg_value("countryName", spec)?;
        }
        VERIFIED_AGE_CREDENTIAL_TYPE => {
            check_number_of_args(1, spec)?;
            let _age_at_least = get_int_arg_value("ageAtLeast", spec)?;
        }
        "VerifiedEmployment" => {
            check_number_of_args(1, spec)?;
            let _employer_name = get_string_arg_value("employerName", spec)?;
        }
        "VerifiedHumanity" => {
            check_number_of_args(0, spec)?;
        }
        _ => {
            return Err(format!(
                "Credential {} is not supported",
                spec.credential_type.as_str()
            ))
        }
    };
    Ok(())
}

fn verify_spec_and_get_group_owner(
    spec: &CredentialSpec,
) -> Result<(CredentialSpec, Principal), String> {
    let (plain_spec, owner) = get_owner_from_spec(spec)?;
    verify_vc_spec(&plain_spec)?;
    Ok((plain_spec, owner))
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

fn verifiable_credential(subject_principal: Principal, credential_spec: &CredentialSpec) -> String {
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
    let (plain_spec, owner) = verify_spec_and_get_group_owner(credential_spec)
        .map_err(IssueCredentialError::UnsupportedCredentialSpec)?;
    GROUPS.with_borrow(|groups| {
        verify_principal_owns_credential(alias_tuple.id_dapp, &plain_spec, owner, groups)
    })?;
    Ok(verifiable_credential(alias_tuple.id_alias, &plain_spec))
}

fn group_name(credential_type: &str) -> Result<String, IssueCredentialError> {
    let name = GROUP_NAME_FOR_CREDENTIAL_TYPE.get(credential_type).ok_or(
        IssueCredentialError::UnsupportedCredentialSpec(credential_type.to_string()),
    )?;
    Ok(name.clone())
}

fn specs_are_equal(spec_1: &CredentialSpec, spec_2: &CredentialSpec) -> bool {
    if spec_1.credential_type != spec_2.credential_type {
        return false;
    }
    let args_1 = spec_1.arguments.clone().unwrap_or_default();
    let args_2 = spec_2.arguments.clone().unwrap_or_default();
    args_1 == args_2
}

fn verify_principal_owns_credential(
    user: Principal,
    credential_spec: &CredentialSpec,
    owner: Principal,
    groups: &GroupsMap,
) -> Result<(), IssueCredentialError> {
    let group_name = group_name(&credential_spec.credential_type)?;
    if let Some(group_record) = groups.get(&(group_name.clone(), owner).into()) {
        if let Some(member_record) = group_record.members.get(&user) {
            let stored_spec: CredentialSpec = OrdCredentialSpec {
                credential_type: credential_spec.credential_type.clone(),
                arguments: member_record.vc_arguments.clone(),
            }
            .into();
            if credential_spec.credential_type == VERIFIED_AGE_CREDENTIAL_TYPE {
                // For VerifiedAge we check that the stored age *implies* the requested spec.
                let requested_min_age = get_int_arg_value("ageAtLeast", credential_spec)
                    .map_err(IssueCredentialError::UnauthorizedSubject)?;
                let stored_age = get_int_arg_value("ageAtLeast", &stored_spec)
                    .map_err(IssueCredentialError::UnauthorizedSubject)?;
                if requested_min_age > stored_age {
                    return Err(IssueCredentialError::UnauthorizedSubject(
                        "user's age doesn't match the requested spec".to_string(),
                    ));
                }
            } else {
                // For all other VCs, we require that stored args are *equal* to the spec's args.
                if !specs_are_equal(credential_spec, &stored_spec) {
                    return Err(IssueCredentialError::UnauthorizedSubject(format!(
                        "user data doesn't match the requested spec:\n got: {:?}\n exp: {:?}:?",
                        *credential_spec, stored_spec
                    )));
                }
            }
            if member_record.membership_status == MembershipStatus::Accepted {
                return Ok(());
            }
        }
    }
    Err(IssueCredentialError::UnauthorizedSubject(format!(
        "user {} has no credential [{}] from issuer {}",
        user, credential_spec.credential_type, owner
    )))
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
