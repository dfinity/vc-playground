use candid::{CandidType, Deserialize, Principal};

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
    pub note: String,
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
pub struct PublicGroupData {
    pub group_name: String,
    pub stats: GroupStats,
    pub is_owner: Option<bool>,
    pub membership_status: Option<MembershipStatus>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct MemberData {
    pub member: Principal,
    pub note: String,
    pub joined_timestamp_ns: u64,
    pub membership_status: MembershipStatus,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct FullGroupData {
    pub group_name: String,
    pub stats: GroupStats,
    pub members: Vec<MemberData>,
}

impl From<FullGroupData> for PublicGroupData {
    fn from(full_data: FullGroupData) -> Self {
        PublicGroupData {
            group_name: full_data.group_name,
            stats: full_data.stats,
            is_owner: None,
            membership_status: None,
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
    AlreadyExists(String),
    NotFound(String),
    Internal(String),
}
