//! Tests related to issue_credential canister call.

use assert_matches::assert_matches;
use candid::{CandidType, Deserialize, Principal};
use canister_tests::framework::{env, get_wasm_path, principal_1, principal_2};
use ic_cdk::api::management_canister::provisional::CanisterId;

use ic_test_state_machine_client::StateMachine;
use lazy_static::lazy_static;
use meta_issuer::groups_api::{
    AddGroupRequest, FullGroupData, GetGroupRequest, GroupsError, JoinGroupRequest,
    ListGroupsRequest, MembershipStatus, PublicGroupData, PublicGroupsData,
    UpdateMembershipRequest,
};
use std::path::PathBuf;

const DUMMY_ROOT_KEY: &str ="308182301d060d2b0601040182dc7c0503010201060c2b0601040182dc7c05030201036100adf65638a53056b2222c91bb2457b0274bca95198a5acbdadfe7fd72178f069bdea8d99e9479d8087a2686fc81bf3c4b11fe275570d481f1698f79d468afe0e57acc1e298f8b69798da7a891bbec197093ec5f475909923d48bfed6843dbed1f";
const DUMMY_II_CANISTER_ID: &str = "rwlgt-iiaaa-aaaaa-aaaaa-cai";
const DUMMY_DERIVATION_ORIGIN: &str = "https://y2aaj-miaaa-aaaad-aacxq-cai.ic0.app";
const DUMMY_FRONTEND_HOSTNAME: &str = "https://y2aaj-miaaa-aaaad-aacxq-cai.ic0.app";

#[derive(CandidType, Deserialize)]
pub struct IssuerInit {
    /// Root of trust for checking canister signatures.
    pub ic_root_key_der: Vec<u8>,
    /// List of canister ids that are allowed to provide id alias credentials.
    pub idp_canister_ids: Vec<Principal>,
    /// The derivation origin to be used by the issuer.
    pub derivation_origin: String,
    /// Frontend hostname to be used by the issuer.
    pub frontend_hostname: String,
}

lazy_static! {
    /// Gzipped Wasm module for the current Early Adopter Issuer build, i.e. the one we're testing
    pub static ref META_ISSUER_WASM: Vec<u8> = {
        let def_path = PathBuf::from("./../").join("meta_issuer.wasm.gz");
        let err = format!("
        Could not find Early Adopter Issuer Wasm module for current build.
        I will look for it at {:?} (note that I run from {:?}).
        You can build the Wasm by running ./build.sh
        ", &def_path,
            &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_|
                "an unknown directory".to_string()));
                get_wasm_path("META_ISSUER_WASM".to_string(), &def_path).expect(&err)

    };

    pub static ref II_WASM: Vec<u8> = {
        let def_path = PathBuf::from("./").join("internet_identity.wasm.gz");
        let err = format!("
        Could not find Internet Identity Wasm module for current build.

        I will look for it at {:?}, and you can specify another path with the environment variable II_WASM (note that I run from {:?}).

        You can download the most recent II-wasm release from 
        https://github.com/dfinity/internet-identity/releases/latest/download/internet_identity_test.wasm.gz
        ", &def_path, &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_| "an unknown directory".to_string()));
        get_wasm_path("II_WASM".to_string(), &def_path).expect(&err)
    };

    pub static ref DUMMY_ISSUER_INIT: IssuerInit = IssuerInit {
        ic_root_key_der: hex::decode(DUMMY_ROOT_KEY).unwrap(),
        idp_canister_ids: vec![Principal::from_text(DUMMY_II_CANISTER_ID).unwrap()],
        derivation_origin: DUMMY_DERIVATION_ORIGIN.to_string(),
        frontend_hostname: DUMMY_FRONTEND_HOSTNAME.to_string(),
    };
}

pub fn install_canister(env: &StateMachine, wasm: Vec<u8>) -> CanisterId {
    let canister_id = env.create_canister(None);
    let arg = candid::encode_one("()").expect("error encoding issuer init arg as candid");
    env.install_canister(canister_id, wasm, arg, None);
    canister_id
}

mod api {
    use super::*;
    use ic_cdk::api::management_canister::main::CanisterId;
    use ic_test_state_machine_client::{call_candid, call_candid_as, CallError, StateMachine};

    pub fn list_groups(
        env: &StateMachine,
        canister_id: CanisterId,
        maybe_sender: Option<Principal>,
        req: ListGroupsRequest,
    ) -> Result<Result<PublicGroupsData, GroupsError>, CallError> {
        match maybe_sender {
            Some(sender) => {
                call_candid_as(env, canister_id, sender, "list_groups", (req,)).map(|(x,)| x)
            }
            None => call_candid(env, canister_id, "list_groups", (req,)).map(|(x,)| x),
        }
    }

    pub fn get_group(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: GetGroupRequest,
    ) -> Result<Result<FullGroupData, GroupsError>, CallError> {
        call_candid_as(env, canister_id, sender, "get_group", (req,)).map(|(x,)| x)
    }

    pub fn add_group(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: AddGroupRequest,
    ) -> Result<Result<FullGroupData, GroupsError>, CallError> {
        call_candid_as(env, canister_id, sender, "add_group", (req,)).map(|(x,)| x)
    }

    pub fn join_group(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: JoinGroupRequest,
    ) -> Result<Result<(), GroupsError>, CallError> {
        call_candid_as(env, canister_id, sender, "join_group", (req,)).map(|(x,)| x)
    }

    pub fn update_membership(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: UpdateMembershipRequest,
    ) -> Result<Result<(), GroupsError>, CallError> {
        call_candid_as(env, canister_id, sender, "update_membership", (req,)).map(|(x,)| x)
    }
}

pub fn install_issuer(env: &StateMachine, init: &IssuerInit) -> CanisterId {
    let canister_id = env.create_canister(None);
    let arg = candid::encode_one(Some(init)).expect("error encoding II installation arg as candid");
    env.install_canister(canister_id, META_ISSUER_WASM.clone(), arg, None);
    canister_id
}

fn add_group(
    group_name: &str,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) -> FullGroupData {
    api::add_group(
        env,
        canister_id,
        caller,
        AddGroupRequest {
            group_name: group_name.to_string(),
        },
    )
    .expect("API call failed")
    .expect("Failed add_group")
}

fn get_group(
    group_name: &str,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) -> FullGroupData {
    api::get_group(
        env,
        canister_id,
        caller,
        GetGroupRequest {
            group_name: group_name.to_string(),
        },
    )
    .expect("API call failed")
    .expect("Failed get_group")
}

fn join_group(
    group_name: &str,
    note: &str,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) {
    api::join_group(
        env,
        canister_id,
        caller,
        JoinGroupRequest {
            group_name: group_name.to_string(),
            note: note.to_string(),
        },
    )
    .expect("API call failed")
    .expect("Failed join_group");
}

fn update_membership(
    group_name: &str,
    member: Principal,
    new_status: MembershipStatus,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) {
    api::update_membership(
        env,
        canister_id,
        caller,
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            member,
            new_status,
        },
    )
    .expect("API call failed")
    .expect("Failed join_group");
}

#[test]
fn should_add_group() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Some group name";
    let group_data = add_group(group_name, principal_1(), &env, canister_id);
    assert_eq!(group_data.group_name, group_name);
}

#[test]
fn should_fail_add_group_if_already_exists() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Some group name";

    // Add the group for the first time
    let group_data = add_group(group_name, principal_1(), &env, canister_id);
    assert_eq!(group_data.group_name, group_name);

    // Try adding again.
    let req = AddGroupRequest {
        group_name: group_name.to_string(),
    };
    let result = api::add_group(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result, Err(GroupsError::AlreadyExists(_)));
}

#[test]
fn should_get_group() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Some group name";
    let created_group_data = add_group(group_name, principal_1(), &env, canister_id);
    let retrieved_group_data = get_group(group_name, principal_1(), &env, canister_id);

    assert_eq!(created_group_data, retrieved_group_data);
}

#[test]
fn should_fail_get_group_if_non_existent() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let req = GetGroupRequest {
        group_name: "Non-existent group".to_string(),
    };
    let result = api::get_group(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result, Err(GroupsError::NotFound(_)));
}

#[test]
fn should_list_groups_anonymously() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());
    let mut group_names = ["first group", "Another group", "yet another group"];
    let mut created_groups: Vec<PublicGroupData> = vec![];
    for group_name in group_names {
        created_groups.push(add_group(group_name, principal_1(), &env, canister_id).into());
    }
    let req = ListGroupsRequest {
        group_name_substring: None,
        only_owned: None,
    };
    let list = api::list_groups(&env, canister_id, None, req)
        .expect("API call failed")
        .expect("Failed to list groups");
    assert_eq!(list.groups.len(), group_names.len());
    let mut retrieved_names: Vec<&str> = list
        .groups
        .iter()
        .map(|data| data.group_name.as_str())
        .collect();
    retrieved_names.sort();
    group_names.sort();
    assert_eq!(retrieved_names, group_names);
    created_groups.sort();
    let mut listed_groups = list.groups.clone();
    listed_groups.sort();
    assert_eq!(listed_groups, created_groups);
    for group in list.groups.iter() {
        assert!(group.membership_status.is_none());
        assert!(group.is_owner.is_none());
    }
}

#[test]
fn should_list_groups_authenticated() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());
    let group_1_owned = "first group";
    let group_2 = "second group";
    let group_3_owned = "third group";
    let owner = principal_1();
    let other_user = principal_2();

    add_group(group_1_owned, owner, &env, canister_id);
    add_group(group_3_owned, owner, &env, canister_id);
    add_group(group_2, other_user, &env, canister_id);

    let note_1 = "first note";
    let note_2 = "second note";
    join_group(group_1_owned, note_1, owner, &env, canister_id);
    join_group(group_2, note_2, owner, &env, canister_id);

    let req = ListGroupsRequest {
        group_name_substring: None,
        only_owned: None,
    };
    let list = api::list_groups(&env, canister_id, Some(owner), req)
        .expect("API call failed")
        .expect("Failed to list groups");
    let mut group_names = [group_1_owned, group_2, group_3_owned];
    assert_eq!(list.groups.len(), group_names.len());
    let mut retrieved_names: Vec<&str> = list
        .groups
        .iter()
        .map(|data| data.group_name.as_str())
        .collect();
    retrieved_names.sort();
    group_names.sort();
    assert_eq!(retrieved_names, group_names);
    for g in &list.groups {
        if g.group_name == group_1_owned {
            assert_matches!(g.membership_status, Some(MembershipStatus::PendingReview));
            assert_matches!(g.is_owner, Some(true));
        } else if g.group_name == group_2 {
            assert_matches!(g.membership_status, Some(MembershipStatus::PendingReview));
            assert_matches!(g.is_owner, Some(false));
        } else if g.group_name == group_3_owned {
            assert_matches!(g.membership_status, None);
            assert_matches!(g.is_owner, Some(true));
        } else {
            panic!("Unexpected group_name: {}", g.group_name)
        }
    }
}

// left:  [PublicGroupData { group_name: "first group", stats: GroupStats { member_count: 1, created_timestamp_ns: 1620328630000000000 }, is_owner: Some(true), membership_status: Some(PendingReview) }, PublicGroupData { group_name: "second group", stats: GroupStats { member_count: 1, created_timestamp_ns: 1620328630000000000 }, is_owner: Some(false), membership_status: Some(PendingReview) }, PublicGroupData { group_name: "third group", stats: GroupStats { member_count: 0, created_timestamp_ns: 1620328630000000000 }, is_owner: Some(true), membership_status: None }]
// right: [PublicGroupData { group_name: "first group", stats: GroupStats { member_count: 0, created_timestamp_ns: 1620328630000000000 }, is_owner: None, membership_status: None }, PublicGroupData { group_name: "second group", stats: GroupStats { member_count: 0, created_timestamp_ns: 1620328630000000000 }, is_owner: None, membership_status: None }, PublicGroupData { group_name: "third group", stats: GroupStats { member_count: 0, created_timestamp_ns: 1620328630000000000 }, is_owner: None, membership_status: None }]
#[test]
fn should_join_group() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Bob's Club";
    let _ = add_group(group_name, principal_1(), &env, canister_id);

    let note = "Alice";
    let alice_principal = principal_2();
    join_group(group_name, note, alice_principal, &env, canister_id);

    let group_data = get_group(group_name, principal_1(), &env, canister_id);

    assert_eq!(group_data.group_name, group_name);
    assert_eq!(group_data.members.len(), 1);
    let member_data = &group_data.members[0];
    assert_eq!(member_data.member, alice_principal);
    assert_eq!(member_data.note, note);
    assert_eq!(
        member_data.membership_status,
        MembershipStatus::PendingReview
    );
}

#[test]
fn should_update_membership() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = add_group(group_name, bob_principal, &env, canister_id);

    let note = "Alice";
    let alice_principal = principal_2();
    join_group(group_name, note, alice_principal, &env, canister_id);

    let group_data = get_group(group_name, principal_1(), &env, canister_id);
    let member_data_before = group_data.members[0].clone();
    assert_eq!(member_data_before.member, alice_principal);
    assert_eq!(member_data_before.note, note);
    assert_eq!(
        member_data_before.membership_status,
        MembershipStatus::PendingReview
    );

    update_membership(
        group_name,
        alice_principal,
        MembershipStatus::Accepted,
        bob_principal,
        &env,
        canister_id,
    );

    let group_data = get_group(group_name, principal_1(), &env, canister_id);
    let member_data_after = group_data.members[0].clone();
    assert_eq!(member_data_after.member, alice_principal);
    assert_eq!(member_data_after.note, note);
    assert_eq!(
        member_data_after.joined_timestamp_ns,
        member_data_before.joined_timestamp_ns
    );
    assert_eq!(
        member_data_after.membership_status,
        MembershipStatus::Accepted
    );
}

#[test]
fn should_fail_update_membership_if_missing_group() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Bob's Club";
    let bob_principal = principal_1();

    let result = api::update_membership(
        &env,
        canister_id,
        bob_principal,
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            member: principal_2(),
            new_status: MembershipStatus::Accepted,
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotFound(e)) if e.contains("group:"));
}

#[test]
fn should_fail_update_membership_if_missing_member() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = add_group(group_name, bob_principal, &env, canister_id);

    let result = api::update_membership(
        &env,
        canister_id,
        bob_principal,
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            member: principal_2(),
            new_status: MembershipStatus::Accepted,
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotFound(e)) if e.contains("member:"));
}

#[test]
fn should_fail_update_membership_if_not_owner() {
    let env = env();
    let canister_id = install_canister(&env, META_ISSUER_WASM.clone());

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = add_group(group_name, bob_principal, &env, canister_id);

    let result = api::update_membership(
        &env,
        canister_id,
        principal_2(), // not the owner
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            member: principal_2(),
            new_status: MembershipStatus::Accepted,
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotAuthorized(e)) if e.contains("owner"));
}
