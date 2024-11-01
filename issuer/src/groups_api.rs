use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;

// "Copies" of ArgumentValue and CredentialSpec which are defined in `ic_verifiable_credentials`-crate,
//  but with VcArguments as HashMap, which does not implement `Ord` and `PartialOrd` (unlike `BTreeMap`)
// TODO: change the definitions in `ic_verifiable_credentials` and remove the copies below.
#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize, Ord, PartialOrd)]
pub enum ArgumentValue {
    String(String),
    Int(i32),
}

impl From<crate::groups_api::CredentialSpec> for ic_verifiable_credentials::issuer_api::CredentialSpec {
    fn from(spec: crate::groups_api::CredentialSpec) -> Self {
        ic_verifiable_credentials::issuer_api::CredentialSpec {
            credential_type: spec.credential_type,
            arguments: spec
                .arguments
                .map(|vc_args| vc_args.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

impl From<crate::groups_api::ArgumentValue> for ic_verifiable_credentials::issuer_api::ArgumentValue {
    fn from(value: crate::groups_api::ArgumentValue) -> Self {
        match value {
            ArgumentValue::String(s) => ic_verifiable_credentials::issuer_api::ArgumentValue::String(s),
            ArgumentValue::Int(i) => ic_verifiable_credentials::issuer_api::ArgumentValue::Int(i),
        }
    }
}

impl From<ic_verifiable_credentials::issuer_api::ArgumentValue> for crate::groups_api::ArgumentValue {
    fn from(value: ic_verifiable_credentials::issuer_api::ArgumentValue) -> Self {
        match value {
            ic_verifiable_credentials::issuer_api::ArgumentValue::String(s) => ArgumentValue::String(s),
            ic_verifiable_credentials::issuer_api::ArgumentValue::Int(i) => ArgumentValue::Int(i),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct CredentialSpec {
    pub credential_type: String,
    pub arguments: Option<VcArguments>,
}

pub type VcArguments = BTreeMap<String, ArgumentValue>;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupType {
    pub group_name: String,
    pub credential_spec: CredentialSpec,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct UserData {
    pub user_nickname: Option<String>,
    pub issuer_nickname: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct SetUserRequest {
    pub user_data: UserData,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ListGroupsRequest {
    pub group_name_substring: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct GetGroupRequest {
    pub group_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct AddGroupRequest {
    pub group_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct JoinGroupRequest {
    pub group_name: String,
    pub owner: Principal,
    pub vc_arguments: Option<VcArguments>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct MembershipUpdate {
    pub member: Principal,
    pub new_status: MembershipStatus,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct UpdateMembershipRequest {
    pub group_name: String,
    pub updates: Vec<MembershipUpdate>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupStats {
    pub member_count: u32,
    pub created_timestamp_ns: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum MembershipStatus {
    PendingReview,
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupTypes {
    pub types: Vec<GroupType>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct PublicGroupData {
    pub group_name: String,
    pub owner: Principal,
    pub issuer_nickname: String,
    pub stats: GroupStats,
    pub membership_status: Option<MembershipStatus>,
    pub vc_arguments: Option<VcArguments>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct MemberData {
    pub member: Principal,
    pub nickname: String,
    pub joined_timestamp_ns: u64,
    pub membership_status: MembershipStatus,
    pub vc_arguments: Option<VcArguments>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct FullGroupData {
    pub group_name: String,
    pub owner: Principal,
    pub issuer_nickname: String,
    pub stats: GroupStats,
    pub members: Vec<MemberData>,
}

impl From<FullGroupData> for PublicGroupData {
    fn from(full_data: FullGroupData) -> Self {
        PublicGroupData {
            group_name: full_data.group_name,
            owner: full_data.owner,
            issuer_nickname: full_data.issuer_nickname,
            stats: full_data.stats,
            membership_status: None,
            vc_arguments: None,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct PublicGroupsData {
    pub groups: Vec<PublicGroupData>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum GroupsError {
    NotAuthorized(String),
    NotAuthenticated(String),
    AlreadyExists(String),
    NotFound(String),
    Internal(String),
}
