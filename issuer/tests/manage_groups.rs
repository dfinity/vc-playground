//! Tests related to group management API.
use assert_matches::assert_matches;
use candid::Principal;
use canister_tests::framework::{env, principal_1, principal_2, test_principal};
use meta_issuer::groups_api::{
    AddGroupRequest, GetGroupRequest, GroupsError, ListGroupsRequest, MemberData, MembershipStatus,
    MembershipUpdate, PublicGroupData, UpdateMembershipRequest,
};
use std::collections::HashMap;
use std::time::Duration;

#[allow(dead_code)]
mod util;
use crate::util::{
    api, do_add_group, do_get_group, do_join_group, do_update_membership, install_issuer,
};

#[test]
fn should_add_group() {
    let env = env();
    let canister_id = install_issuer(&env, None);
    let caller = principal_1();

    let group_name = "Some group name";
    let group_data = do_add_group(group_name, caller, &env, canister_id);
    assert_eq!(group_data.group_name, group_name);
    assert_eq!(group_data.members.len(), 1);
    let member_data = &group_data.members[0];
    let expected_member_data = MemberData {
        member: caller,
        note: "owner".to_string(),
        joined_timestamp_ns: member_data.joined_timestamp_ns,
        membership_status: MembershipStatus::Accepted,
    };
    assert_eq!(*member_data, expected_member_data);
}

#[test]
fn should_fail_add_group_if_already_exists() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Some group name";

    // Add the group for the first time
    let group_data = do_add_group(group_name, principal_1(), &env, canister_id);
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
    let canister_id = install_issuer(&env, None);

    let group_name = "Some group name";
    let created_group_data = do_add_group(group_name, principal_1(), &env, canister_id);
    let retrieved_group_data = do_get_group(group_name, principal_1(), &env, canister_id);

    assert_eq!(created_group_data, retrieved_group_data);
}

#[test]
fn should_fail_get_group_if_non_existent() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let req = GetGroupRequest {
        group_name: "Non-existent group".to_string(),
    };
    let result = api::get_group(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result, Err(GroupsError::NotFound(_)));
}

#[test]
fn should_list_groups_anonymously() {
    let env = env();
    let canister_id = install_issuer(&env, None);
    let mut group_names = ["first group", "Another group", "yet another group"];
    let mut created_groups: Vec<PublicGroupData> = vec![];
    for group_name in group_names {
        created_groups.push(do_add_group(group_name, principal_1(), &env, canister_id).into());
    }
    let req = ListGroupsRequest {
        group_name_substring: None,
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
    let canister_id = install_issuer(&env, None);
    let group_1_owned = "first group";
    let group_2 = "second group";
    let group_3_owned = "third group";
    let owner = principal_1();
    let other_user = principal_2();

    do_add_group(group_1_owned, owner, &env, canister_id);
    do_add_group(group_3_owned, owner, &env, canister_id);
    do_add_group(group_2, other_user, &env, canister_id);

    let note_1 = "first note";
    let note_2 = "second note";
    do_join_group(group_1_owned, note_1, owner, &env, canister_id);
    do_join_group(group_2, note_2, owner, &env, canister_id);

    let req = ListGroupsRequest {
        group_name_substring: None,
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
            assert_matches!(g.membership_status, Some(MembershipStatus::Accepted));
            assert_matches!(g.is_owner, Some(true));
        } else if g.group_name == group_2 {
            assert_matches!(g.membership_status, Some(MembershipStatus::PendingReview));
            assert_matches!(g.is_owner, Some(false));
        } else if g.group_name == group_3_owned {
            assert_matches!(g.membership_status, Some(MembershipStatus::Accepted));
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
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let _ = do_add_group(group_name, principal_1(), &env, canister_id);

    let note = "Alice";
    let alice_principal = principal_2();
    do_join_group(group_name, note, alice_principal, &env, canister_id);

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);

    assert_eq!(group_data.group_name, group_name);
    // member[0] is the owner, member[1] is the added member Alice
    assert_eq!(group_data.members.len(), 2);
    let member_data = &group_data.members[1];
    assert_eq!(member_data.member, alice_principal);
    assert_eq!(member_data.note, note);
    assert_eq!(
        member_data.membership_status,
        MembershipStatus::PendingReview
    );
}

#[test]
fn should_update_membership_single_member() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = do_add_group(group_name, bob_principal, &env, canister_id);

    let note = "Alice";
    let alice_principal = principal_2();
    do_join_group(group_name, note, alice_principal, &env, canister_id);

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);
    // member[0] is the owner, member[1] is the added member Alice
    let member_data_before = group_data.members[1].clone();
    assert_eq!(member_data_before.member, alice_principal);
    assert_eq!(member_data_before.note, note);
    assert_eq!(
        member_data_before.membership_status,
        MembershipStatus::PendingReview
    );

    env.advance_time(Duration::from_secs(2));
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member: alice_principal,
            new_status: MembershipStatus::Accepted,
        }],
        bob_principal,
        &env,
        canister_id,
    );

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);
    let member_data_after = group_data.members[1].clone();
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
fn should_update_membership_multiple_members() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    // creator automatically joins the group as "owner"
    let bob_note = "owner";
    let bob_principal = principal_1();
    let _ = do_add_group(group_name, bob_principal, &env, canister_id);

    env.advance_time(Duration::from_secs(2));
    let alice_note = "Alice";
    let alice_principal = principal_2();
    do_join_group(group_name, alice_note, alice_principal, &env, canister_id);

    env.advance_time(Duration::from_secs(2));
    let eve_note = "Eve";
    let eve_principal = test_principal(42);
    do_join_group(group_name, eve_note, eve_principal, &env, canister_id);

    let mut timestamps: HashMap<Principal, u64> = HashMap::new();

    let group_data_before = do_get_group(group_name, principal_1(), &env, canister_id);
    assert_eq!(group_data_before.members.len(), 3);
    for m in group_data_before.members {
        timestamps.insert(m.member, m.joined_timestamp_ns);
        if m.member == alice_principal {
            assert_eq!(m.note, alice_note);
            assert_eq!(m.membership_status, MembershipStatus::PendingReview);
        } else if m.member == bob_principal {
            assert_eq!(m.note, bob_note);
            assert_eq!(m.membership_status, MembershipStatus::Accepted);
        } else if m.member == eve_principal {
            assert_eq!(m.note, eve_note);
            assert_eq!(m.membership_status, MembershipStatus::PendingReview);
        } else {
            panic!("Unexpected member {}", m.member);
        }
    }

    env.advance_time(Duration::from_secs(2));
    do_update_membership(
        group_name,
        vec![
            MembershipUpdate {
                member: alice_principal,
                new_status: MembershipStatus::Accepted,
            },
            MembershipUpdate {
                member: bob_principal,
                new_status: MembershipStatus::Rejected,
            },
        ],
        bob_principal,
        &env,
        canister_id,
    );

    let group_data_after = do_get_group(group_name, principal_1(), &env, canister_id);
    assert_eq!(group_data_after.members.len(), 3);
    for m in group_data_after.members {
        assert_eq!(
            m.joined_timestamp_ns,
            *timestamps.get(&m.member).expect("Missing member")
        );
        if m.member == alice_principal {
            assert_eq!(m.note, alice_note);
            assert_eq!(m.membership_status, MembershipStatus::Accepted);
        } else if m.member == bob_principal {
            assert_eq!(m.note, bob_note);
            assert_eq!(m.membership_status, MembershipStatus::Rejected);
        } else if m.member == eve_principal {
            assert_eq!(m.note, eve_note);
            assert_eq!(m.membership_status, MembershipStatus::PendingReview);
        } else {
            panic!("Unexpected member {}", m.member);
        }
    }
}

#[test]
fn should_update_membership_multiple_times() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let bob_note = "owner";
    let _ = do_add_group(group_name, bob_principal, &env, canister_id);

    let alice_note = "Alice";
    let alice_principal = principal_2();
    do_join_group(group_name, alice_note, alice_principal, &env, canister_id);

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);
    // member[0] is the owner, member[1] is the added member Alice
    let bob_data_before = group_data.members[0].clone();
    assert_eq!(bob_data_before.member, bob_principal);
    assert_eq!(bob_data_before.note, bob_note);
    assert_eq!(
        bob_data_before.membership_status,
        MembershipStatus::Accepted
    );
    let alice_data_before = group_data.members[1].clone();
    assert_eq!(alice_data_before.member, alice_principal);
    assert_eq!(alice_data_before.note, alice_note);
    assert_eq!(
        alice_data_before.membership_status,
        MembershipStatus::PendingReview
    );

    env.advance_time(Duration::from_secs(2));
    // Update memberships for the first time.
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member: bob_principal,
            new_status: MembershipStatus::Rejected,
        }],
        bob_principal,
        &env,
        canister_id,
    );
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member: alice_principal,
            new_status: MembershipStatus::Accepted,
        }],
        bob_principal,
        &env,
        canister_id,
    );

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);
    let bob_data_after = group_data.members[0].clone();
    assert_eq!(bob_data_after.member, bob_principal);
    assert_eq!(bob_data_after.note, bob_note);
    assert_eq!(
        bob_data_after.joined_timestamp_ns,
        bob_data_before.joined_timestamp_ns
    );
    assert_eq!(bob_data_after.membership_status, MembershipStatus::Rejected);

    let alice_data_after = group_data.members[1].clone();
    assert_eq!(alice_data_after.member, alice_principal);
    assert_eq!(alice_data_after.note, alice_note);
    assert_eq!(
        alice_data_after.joined_timestamp_ns,
        alice_data_before.joined_timestamp_ns
    );
    assert_eq!(
        alice_data_after.membership_status,
        MembershipStatus::Accepted
    );

    // Update memberships another time.
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member: bob_principal,
            new_status: MembershipStatus::Accepted,
        }],
        bob_principal,
        &env,
        canister_id,
    );
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member: alice_principal,
            new_status: MembershipStatus::Rejected,
        }],
        bob_principal,
        &env,
        canister_id,
    );

    let group_data = do_get_group(group_name, principal_1(), &env, canister_id);
    let bob_data_after = group_data.members[0].clone();
    assert_eq!(bob_data_after.member, bob_principal);
    assert_eq!(bob_data_after.note, bob_note);
    assert_eq!(
        bob_data_after.joined_timestamp_ns,
        bob_data_before.joined_timestamp_ns
    );
    assert_eq!(bob_data_after.membership_status, MembershipStatus::Accepted);

    let alice_data_after = group_data.members[1].clone();
    assert_eq!(alice_data_after.member, alice_principal);
    assert_eq!(alice_data_after.note, alice_note);
    assert_eq!(
        alice_data_after.joined_timestamp_ns,
        alice_data_before.joined_timestamp_ns
    );
    assert_eq!(
        alice_data_after.membership_status,
        MembershipStatus::Rejected
    );
}

#[test]
fn should_fail_update_membership_if_missing_group() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let bob_principal = principal_1();

    let result = api::update_membership(
        &env,
        canister_id,
        bob_principal,
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            updates: vec![MembershipUpdate {
                member: principal_2(),
                new_status: MembershipStatus::Accepted,
            }],
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotFound(e)) if e.contains("group:"));
}

#[test]
fn should_fail_update_membership_if_missing_member() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = do_add_group(group_name, bob_principal, &env, canister_id);

    let result = api::update_membership(
        &env,
        canister_id,
        bob_principal,
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            updates: vec![MembershipUpdate {
                member: principal_2(),
                new_status: MembershipStatus::Accepted,
            }],
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotFound(e)) if e.contains("member:"));
}

#[test]
fn should_fail_update_membership_if_not_owner() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let group_name = "Bob's Club";
    let bob_principal = principal_1();
    let _ = do_add_group(group_name, bob_principal, &env, canister_id);

    let result = api::update_membership(
        &env,
        canister_id,
        principal_2(), // not the owner
        UpdateMembershipRequest {
            group_name: group_name.to_string(),
            updates: vec![MembershipUpdate {
                member: principal_2(),
                new_status: MembershipStatus::Accepted,
            }],
        },
    )
    .expect("API call failed");

    assert_matches!(result, Err(GroupsError::NotAuthorized(e)) if e.contains("owner"));
}
