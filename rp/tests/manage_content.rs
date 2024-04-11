//! Tests related to group management API.
use assert_matches::assert_matches;
use candid::Principal;
use canister_tests::framework::{env, principal_1, principal_2, test_principal};
use relying_party::rp_api::{ContentData, ContentError, IssuerData, RpInit, ValidateVpRequest};
use std::collections::{HashMap, HashSet};
use vc_util::issuer_api::{ArgumentValue, CredentialSpec};
use vc_util::II_ISSUER_URL;

#[allow(dead_code)]
mod util;
use crate::util::{
    api, do_add_exclusive_content, do_list_exclusive_content, do_list_images, install_rp,
};

#[test]
fn should_list_images() {
    let env = env();
    let canister_id = install_rp(&env, None);

    let list = do_list_images(&env, canister_id);
    assert_eq!(list.images.len(), 11);

    let mut img_set = HashSet::new();
    for img_data in &list.images {
        img_set.insert(img_data.url.clone());
    }
    for file_name in [
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
    ] {
        assert!(img_set.contains(&format!("/images/{}", file_name)));
    }
    assert_eq!(img_set.len(), 11);
}

#[test]
fn should_add_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();
    let group_owner = principal_2();

    let content_name = "Some content name";
    let group_name = "Some group name";
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        group_name,
        group_owner,
        caller,
        &env,
        canister_id,
    );
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_group_name: group_name.to_string(),
        credential_group_owner: group_owner,
    };
    assert_eq!(content_data, expected_content_data);
}

#[test]
fn should_list_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();
    let group_owner = principal_2();

    let content_name = "Some content name";
    let group_name = "Some group name";
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        group_name,
        group_owner,
        caller,
        &env,
        canister_id,
    );
    let content_list = do_list_exclusive_content(&env, None, canister_id);
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_group_name: group_name.to_string(),
        credential_group_owner: group_owner,
    };
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);
}

#[test]
fn should_list_exclusive_content_multiple_items() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = [principal_1(), principal_2(), test_principal(42)];
    let group_owner = principal_2();

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
            group_owner,
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
            credential_group_owner: group_owner,
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

fn get_validate_vp_request_and_rp_init() -> (ValidateVpRequest, RpInit) {
    // Example data for the test generated via issuer's should_issue_share_and_validate_e2e()-test
    let ic_root_key_der = vec![
        48, 129, 130, 48, 29, 6, 13, 43, 6, 1, 4, 1, 130, 220, 124, 5, 3, 1, 2, 1, 6, 12, 43, 6, 1,
        4, 1, 130, 220, 124, 5, 3, 2, 1, 3, 97, 0, 173, 246, 86, 56, 165, 48, 86, 178, 34, 44, 145,
        187, 36, 87, 176, 39, 75, 202, 149, 25, 138, 90, 203, 218, 223, 231, 253, 114, 23, 143, 6,
        155, 222, 168, 217, 158, 148, 121, 216, 8, 122, 38, 134, 252, 129, 191, 60, 75, 17, 254,
        39, 85, 112, 212, 129, 241, 105, 143, 121, 212, 104, 175, 224, 229, 122, 204, 30, 41, 143,
        139, 105, 121, 141, 167, 168, 145, 187, 236, 25, 112, 147, 236, 95, 71, 89, 9, 146, 61, 72,
        191, 237, 104, 67, 219, 237, 31,
    ];
    let issuer_canister_id =
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("wrong principal");
    let ii_canister_id =
        Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").expect("wrong principal");
    let _current_time_sec = 1620328630;
    let issuer_origin = "https://metaissuer.vc/";
    let mut args = HashMap::new();
    args.insert(
        "groupName".to_string(),
        ArgumentValue::String("Dummy group".to_string()),
    );
    args.insert(
        "owner".to_string(),
        ArgumentValue::String(
            "6epth-hmqup-wz4mv-svl2m-mhcbb-24skq-tbdhq-2txct-2qugv-xuzva-eqe".to_string(),
        ),
    );

    let req = ValidateVpRequest {
        vp_jwt: "eyJ0eXAiOiJKV1QiLCJhbGciOiJub25lIn0.eyJpc3MiOiJkaWQ6aWNwOm43ZTRnLTN2ZGZ0LXF3MnZ3LWt4Nnl0LXUyYmFsLTRzdG1tLW5vZDNiLTM1ejR6LXdzb2o3LW00azY2LWhxZSIsInZwIjp7IkBjb250ZXh0IjoiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiLCJ0eXBlIjoiVmVyaWZpYWJsZVByZXNlbnRhdGlvbiIsInZlcmlmaWFibGVDcmVkZW50aWFsIjpbImV5SnFkMnNpT25zaWEzUjVJam9pYjJOMElpd2lZV3huSWpvaVNXTkRjeUlzSW1zaU9pSk5SSGQzUkVGWlMwdDNXVUpDUVVkRWRVVk5Ra0ZuVFhOQlFXOUJRVUZCUVVGQlFVRkJRVVZDYTJ4dWNtSjBZMlI2YjJoeE1rNWxha1UwZVhjNVJ6UlNRVlZTZEZJeE1sZGpkbXByWVZKc1EydHlieUo5TENKcmFXUWlPaUprYVdRNmFXTndPbkozYkdkMExXbHBZV0ZoTFdGaFlXRmhMV0ZoWVdGaExXTmhhU0lzSW1Gc1p5STZJa2xqUTNNaWZRLmV5SmxlSEFpT2pFMk1qQXpNamsxTXpBc0ltbHpjeUk2SW1oMGRIQnpPaTh2YVdSbGJuUnBkSGt1YVdNd0xtRndjQzhpTENKdVltWWlPakUyTWpBek1qZzJNekFzSW1wMGFTSTZJbVJoZEdFNmRHVjRkQzl3YkdGcGJqdGphR0Z5YzJWMFBWVlVSaTA0TEhScGJXVnpkR0Z0Y0Y5dWN6b3hOakl3TXpJNE5qTXdNREF3TURBd01EQXdMR0ZzYVdGelgyaGhjMmc2TmpReVlUUmhaVEUyT0dFMU5qVTBZelJoT1dSaU56RXpZMlpoTTJZeE9USm1NVGRsTW1Jd1lqWXhaR1UxTTJOa1lXTmxaVEJsTjJJME9XUTVPVEppTnlJc0luTjFZaUk2SW1ScFpEcHBZM0E2YmpkbE5HY3RNM1prWm5RdGNYY3lkbmN0YTNnMmVYUXRkVEppWVd3dE5ITjBiVzB0Ym05a00ySXRNelY2TkhvdGQzTnZhamN0YlRSck5qWXRhSEZsSWl3aWRtTWlPbnNpUUdOdmJuUmxlSFFpT2lKb2RIUndjem92TDNkM2R5NTNNeTV2Y21jdk1qQXhPQzlqY21Wa1pXNTBhV0ZzY3k5Mk1TSXNJblI1Y0dVaU9sc2lWbVZ5YVdacFlXSnNaVU55WldSbGJuUnBZV3dpTENKSmJuUmxjbTVsZEVsa1pXNTBhWFI1U1dSQmJHbGhjeUpkTENKamNtVmtaVzUwYVdGc1UzVmlhbVZqZENJNmV5SkpiblJsY201bGRFbGtaVzUwYVhSNVNXUkJiR2xoY3lJNmV5Sm9ZWE5KWkVGc2FXRnpJam9pZEhkNGVXOHRjRFpuYVhNdFlYcHhOSEV0WkROME5UY3RkM3BsWmpZdFp6Wmphbm90ZWpaMWNYWXRiWFIwTjNrdFkydzNhM2N0WnpkclpHVXRkWEZsSW4xOWZYMC4yZG4zb210alpYSjBhV1pwWTJGMFpWa0IxOW5aOTZKa2RISmxaWU1CZ3dHREFZTUNTR05oYm1semRHVnlnd0dEQVlNQ1NnQUFBQUFBQUFBQUFRR0RBWU1CZ3dHREFrNWpaWEowYVdacFpXUmZaR0YwWVlJRFdDRGJuUU54U1ppMk9TSzhKcXM0MWJ3QTZoS0ZYa0ROdjdNQUo2dGFpVmtlWVlJRVdDRFN6UDhfekYycE1ON0E2RW9sUWVaWVVXdExzVkFlY09pMEVPYVlhZ1Y5WUlJRVdDRDhwblJOV3E1dWo4UVphZ3hROERma0ZobmdIbGxHUkhfTU44MGhBTEJyam9JRVdDQmRqcXdlckMtZE5TUVdDMXJXQUYtWlllYm5pTU96X3h2OGExZndfbWpndDRJRVdDQ1RBTHUxeXIzNkxydmNwOHRMeVR1ME51S2ltOGtZcU85WU9VRkJVYlVwSW9JRVdDQjZWdDFWNDFMaHNFR1lKYlRTampQRTBfckY2Yl9DYW1XQy1JVl8xbnM3dm9JRVdDQjQ0TGh3TzRJdEVsNnNRUFFHQkZJYnJYdUNEOGZiZG14QXROOGl5WXYwOW9JRVdDQVVlU1pvQ296TGhlZ1dpMFlsSktWMzRsWGs1VDcyYWdqbXA5VTZMV0MtNDRNQmdnUllJRFZUOWxnZFU3UGRKMlJ2WWNpdzJnc01PSUc0VjhiY1BFZnFhSWpCRS10cmd3SkVkR2x0WllJRFNZQzRydFRkaWFTLUZtbHphV2R1WVhSMWNtVllNSmRObGVTYnpIc2lpVWhZamkwQ1FYNlN5azVpdndEeVNkTlp6eXRLX1l3Q1JCZkg5aFMzbDQ4dFNBUTk4OWhCaFdSMGNtVmxnd0dDQkZnZ1BRMHgxdnk4RmxReWw4Tk9GNG8zbnU0emZaa2laTUhOcnZFbnRPc1BWT1NEQWtOemFXZURBbGdnS1BYTXdpS2xadUp5UzRaYkFqR0VNeDFRODdaY3czdG4tbHJCVnlDbTdieURBWU1DV0NEZERxVklZMTJ4Z0tPMHlxNng5SUV1NHl5Nml3WjJLemxTUFE2TjZDMTUwWUlEUUlJRVdDQ0VjWldsS3JaQkpPelhPSW9FQkZ4VzZvT21ZX1RfYkplaVhIWFBVVTYwNEEiLCJleUpxZDJzaU9uc2lhM1I1SWpvaWIyTjBJaXdpWVd4bklqb2lTV05EY3lJc0ltc2lPaUpOUkhkM1JFRlpTMHQzV1VKQ1FVZEVkVVZOUWtGblRYTkJRVzlCUVVGQlFVRkJRVUZCVVVWQ05WVXlSMlF4VTJ0bFVIQk9MWEJQZDFsa1pGZENjelZuWDNwYWVGcFlZbmxLVFdKRU9GQnRTV2RYZHlKOUxDSnJhV1FpT2lKa2FXUTZhV053T25KeWEyRm9MV1p4WVdGaExXRmhZV0ZoTFdGaFlXRnhMV05oYVNJc0ltRnNaeUk2SWtsalEzTWlmUS5leUpsZUhBaU9qRTJNakF6TWprMU16QXNJbWx6Y3lJNkltaDBkSEJ6T2k4dmJXVjBZV2x6YzNWbGNpNTJZeThpTENKdVltWWlPakUyTWpBek1qZzJNekFzSW1wMGFTSTZJbVJoZEdFNmRHVjRkQzl3YkdGcGJqdGphR0Z5YzJWMFBWVlVSaTA0TEdsemMzVmxjanBvZEhSd2N6b3ZMMjFsZEdGcGMzTjFaWEl1ZG1Nc2RHbHRaWE4wWVcxd1gyNXpPakUyTWpBek1qZzJNekF3TURBd01EQXdNREFzYzNWaWFtVmpkRHAwZDNoNWJ5MXdObWRwY3kxaGVuRTBjUzFrTTNRMU55MTNlbVZtTmkxbk5tTnFlaTE2Tm5WeGRpMXRkSFEzZVMxamJEZHJkeTFuTjJ0a1pTMTFjV1VpTENKemRXSWlPaUprYVdRNmFXTndPblIzZUhsdkxYQTJaMmx6TFdGNmNUUnhMV1F6ZERVM0xYZDZaV1kyTFdjMlkycDZMWG8yZFhGMkxXMTBkRGQ1TFdOc04ydDNMV2MzYTJSbExYVnhaU0lzSW5aaklqcDdJa0JqYjI1MFpYaDBJam9pYUhSMGNITTZMeTkzZDNjdWR6TXViM0puTHpJd01UZ3ZZM0psWkdWdWRHbGhiSE12ZGpFaUxDSjBlWEJsSWpwYklsWmxjbWxtYVdGaWJHVkRjbVZrWlc1MGFXRnNJaXdpVm1WeWFXWnBaV1JOWlcxaVpYSWlYU3dpWTNKbFpHVnVkR2xoYkZOMVltcGxZM1FpT25zaVZtVnlhV1pwWldSTlpXMWlaWElpT25zaVozSnZkWEJPWVcxbElqb2lSSFZ0YlhrZ1ozSnZkWEFpTENKdmQyNWxjaUk2SWpabGNIUm9MV2h0Y1hWd0xYZDZORzEyTFhOMmJESnRMVzFvWTJKaUxUSTBjMnR4TFhSaVpHaHhMVEowZUdOMExUSnhkV2QyTFhoMWVuWmhMV1Z4WlNKOWZYMTkuMmRuM29tdGpaWEowYVdacFkyRjBaVmtCMTluWjk2SmtkSEpsWllNQmd3R0RBWU1DU0dOaGJtbHpkR1Z5Z3dHREFZSUVXQ0M4S3RZbmNDby1RNVZpb2w4bFlXVGUzSDI4bDhBOUZvNjhUdHc2bnYxeXBvTUNTZ0FBQUFBQUFBQUJBUUdEQVlNQmd3R0RBazVqWlhKMGFXWnBaV1JmWkdGMFlZSURXQ0JDRXBmSUpVM0xhVE5aNWEwa1hhd3R2OXlJcTlPOS1sa3VnSVF2X1dqd3dJSUVXQ0RTelA4X3pGMnBNTjdBNkVvbFFlWllVV3RMc1ZBZWNPaTBFT2FZYWdWOVlJSUVXQ0FnV1dhLWhWSDNmM01UelNaQXNDOGhRQmp0QVhBM2xEeXloY21lUkpkX2RJSUVXQ0NRSWg4Y0JjaTFfYUFEdU1QUGJzUFlRaE5DZ0ZCc0VaX1ZHWnR3Q3dDaTdJSUVXQ0I2VnQxVjQxTGhzRUdZSmJUU2pqUEUwX3JGNmJfQ2FtV0MtSVZfMW5zN3ZvSUVXQ0NFOGJ0TndYYTR4enR2NWFmMGxQNGxSSzFlWTkwdGhfUktlNmFqaVhpLW9ZSUVXQ0IyWnI3THJweEE1RWFyNkVqd2FCYTlWd0toRmx1U0J2VXlaajhldVRQZ1FvTUJnZ1JZSURWVDlsZ2RVN1BkSjJSdlljaXcyZ3NNT0lHNFY4YmNQRWZxYUlqQkUtdHJnd0pFZEdsdFpZSURTWUM0cnRUZGlhUy1GbWx6YVdkdVlYUjFjbVZZTUphMnFtendnM0drakQwcENVNE5haEIzaC1IMFhRd3ZUMVdsUjRLS3NtMnBQTmVnWDZyYmhwNGFENkhaZGV2VGQyUjBjbVZsZ3dHQ0JGZ2dUcE9SeW40Nzd4SXpXV0lBM0NiRnpGNXBrTVZieUFRd3JId01wQWtXclJTREFrTnphV2VEQWxnZ09ZTGItb1J1bzRuMG1ucWZkSkY5b05lZlZpSFk3TFYzclZ0ZEtoazMxdy1EQWxnZ0Zoa3lwQUNMMkh0bjE2SWdUMDl1VUZTSTl4UVJSV3R1V2N6ekx3LUZtN0dDQTBBIl19fQ.".to_string(),
        effective_vc_subject: Principal::from_text("n7e4g-3vdft-qw2vw-kx6yt-u2bal-4stmm-nod3b-35z4z-wsoj7-m4k66-hqe").expect("wrong principal"),
        credential_spec: CredentialSpec { credential_type: "VerifiedMember".to_string(), arguments: Some(args) },
        issuer_origin: issuer_origin.to_string(),
        issuer_canister_id: Some(issuer_canister_id),
    };

    let rp_init = RpInit {
        ic_root_key_der,
        ii_origin: II_ISSUER_URL.to_string(),
        ii_canister_id,
        issuers: vec![IssuerData{ origin: issuer_origin.to_string(), canister_id: issuer_canister_id }],
    };
    (req, rp_init)
}

#[test]
fn should_verify_ii_vp() {
    let (req, rp_init) = get_validate_vp_request_and_rp_init();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert!(result.is_ok(), "Validation failed: {:?}", result.err());
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_issuer_origin() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.issuer_origin = "http://wrong.origin".to_string();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("issuer not supported"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_issuer_canister_id() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.issuer_canister_id = Some(principal_1());
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("wrong issuer canister id"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_effective_vc_subject() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.effective_vc_subject = principal_1();
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("unexpected vc subject"));
}

#[test]
fn should_fail_verify_ii_vp_with_wrong_credential_spec() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.credential_spec = CredentialSpec {
        credential_type: "WrongType".to_string(),
        arguments: None,
    };
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("credential_type"));
}

#[test]
fn should_fail_verify_ii_vp_with_invalid_vp() {
    let (mut req, rp_init) = get_validate_vp_request_and_rp_init();
    req.vp_jwt.insert(42, 'a');
    // Setup the environment and the RP-canister with the matching configuration.
    let env = env();
    let canister_id = install_rp(&env, Some(rp_init));

    let result =
        api::validate_ii_vp(&env, canister_id, principal_1(), req).expect("API call failed");
    assert_matches!(result,
        Err(ContentError::NotAuthorized(e)) if e.contains("failed parsing presentation claims"));
}
