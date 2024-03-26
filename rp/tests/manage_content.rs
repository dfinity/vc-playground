//! Tests related to group management API.
use canister_tests::framework::{env, principal_1, principal_2, test_principal};
use relying_party::rp_api::ContentData;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
mod util;
use crate::util::{
    do_add_exclusive_content, do_list_exclusive_content, do_list_images, install_rp,
};

#[test]
fn should_list_images() {
    let env = env();
    let canister_id = install_rp(&env, None);

    let list = do_list_images(&env, canister_id);
    assert_eq!(list.images.len(), 14);

    let mut img_set = HashSet::new();
    for img_data in &list.images {
        img_set.insert(img_data.url.clone());
    }
    assert!(img_set.contains("/images/Rectangle05.png"));
    assert!(img_set.contains("/images/Rectangle10.png"));
    assert!(img_set.contains("/images/Rectangle12.png"));
    assert_eq!(img_set.len(), 14);
}

#[test]
fn should_add_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();

    let content_name = "Some content name";
    let group_name = "Some group name";
    let url = "http://example.com";
    let content_data =
        do_add_exclusive_content(content_name, url, group_name, caller, &env, canister_id);
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_group_name: group_name.to_string(),
    };
    assert_eq!(content_data, expected_content_data);
}

#[test]
fn should_list_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();

    let content_name = "Some content name";
    let group_name = "Some group name";
    let url = "http://example.com";
    let content_data =
        do_add_exclusive_content(content_name, url, group_name, caller, &env, canister_id);
    let content_list = do_list_exclusive_content(&env, None, canister_id);
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_group_name: group_name.to_string(),
    };
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);
}

#[test]
fn should_list_exclusive_content_multiple_items() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = [principal_1(), principal_2(), test_principal(42)];

    let content_name = [
        "Some content name 1",
        "content name 2",
        "another content name",
    ];
    let group_name = ["group name 1", "other group name", "yet another group"];
    let url = ["http://example_1.com", "other.url", "another url"];
    let mut expected_list = HashMap::new();
    for i in 0..3 {
        let content_data = do_add_exclusive_content(
            content_name[i],
            url[i],
            group_name[i],
            caller[i],
            &env,
            canister_id,
        );
        let expected_content_data = ContentData {
            owner: caller[i],
            content_name: content_name[i].to_string(),
            created_timestamp_ns: content_data.created_timestamp_ns,
            url: url[i].to_string(),
            credential_group_name: group_name[i].to_string(),
        };
        expected_list.insert(content_name[i].to_string(), expected_content_data);
    }
    let content_list = do_list_exclusive_content(&env, None, canister_id);

    assert_eq!(content_list.content_items.len(), content_name.len());
    for item in content_list.content_items {
        let expected = expected_list
            .remove_entry(&item.content_name)
            .expect("Unexpected item");
        assert_eq!(expected.1, item);
    }
}
