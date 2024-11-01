//! Tests related to group management API.
use assert_matches::assert_matches;
use candid::Principal;
use canister_tests::framework::{env, principal_1, principal_2, test_principal};
use ic_verifiable_credentials::issuer_api::{ArgumentValue, CredentialSpec};
use relying_party::rp_api::{ContentData, ContentError, IssuerData, RpInit, ValidateVpRequest};
use std::collections::{HashMap, HashSet};
use ic_verifiable_credentials::II_ISSUER_URL;

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
    let credential_issuer = principal_2();

    let content_name = "Some content name";
    let credential_spec = CredentialSpec {
        credential_type: "VerifiedData".to_string(),
        arguments: None,
    };
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        &credential_spec,
        credential_issuer,
        caller,
        &env,
        canister_id,
    );
    let expected_content_data = ContentData {
        owner: caller,
        content_name: content_name.to_string(),
        created_timestamp_ns: content_data.created_timestamp_ns,
        url: url.to_string(),
        credential_spec,
        credential_issuer,
    };
    assert_eq!(content_data, expected_content_data);
}

#[test]
fn should_list_exclusive_content() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = principal_1();
    let credential_issuer = principal_2();

    let content_name = "Some content name";
    let credential_spec = CredentialSpec {
        credential_type: "VerifiedData".to_string(),
        arguments: None,
    };
    let url = "http://example.com";
    let content_data = do_add_exclusive_content(
        content_name,
        url,
        &credential_spec,
        credential_issuer,
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
        credential_spec,
        credential_issuer,
    };
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);
}

#[test]
fn should_list_exclusive_content_multiple_items() {
    let env = env();
    let canister_id = install_rp(&env, None);
    let caller = [principal_1(), principal_2(), test_principal(42)];
    let credential_issuer = principal_2();

    let content_name = [
        "Some content name 1",
        "content name 2",
        "another content name",
    ];
    let credential_spec = [
        CredentialSpec {
            credential_type: "VerifiedData1".to_string(),
            arguments: None,
        },
        CredentialSpec {
            credential_type: "VerifiedData2".to_string(),
            arguments: None,
        },
        CredentialSpec {
            credential_type: "VerifiedData3".to_string(),
            arguments: None,
        },
    ];

    let url = ["http://example_1.com", "other.url", "another url"];
    let mut expected_list = HashMap::new();
    for i in 0..3 {
        let content_data = do_add_exclusive_content(
            content_name[i],
            url[i],
            &credential_spec[i],
            credential_issuer,
            caller[i],
            &env,
            canister_id,
        );
        let expected_content_data = ContentData {
            owner: caller[i],
            content_name: content_name[i].to_string(),
            created_timestamp_ns: content_data.created_timestamp_ns,
            url: url[i].to_string(),
            credential_spec: credential_spec[i].clone(),
            credential_issuer,
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
    // Example data for the test generated via issuer's should_issue_vc_and_validate_e2e()-test
    let ic_root_key_der = vec![
        48, 129, 130, 48, 29, 6, 13, 43, 6, 1, 4, 1, 130, 220, 124, 5, 3, 1, 2, 1, 6, 12, 43, 6, 1,
        4, 1, 130, 220, 124, 5, 3, 2, 1, 3, 97, 0, 182, 178, 3, 13, 226, 142, 189, 144, 157, 71,
        102, 110, 73, 71, 153, 170, 25, 223, 99, 208, 196, 189, 205, 27, 212, 184, 134, 206, 234,
        96, 58, 18, 40, 241, 19, 231, 253, 110, 171, 99, 241, 114, 182, 84, 61, 111, 118, 215, 2,
        116, 58, 193, 247, 189, 100, 124, 201, 61, 174, 66, 185, 187, 244, 5, 32, 91, 129, 66, 255,
        1, 5, 2, 17, 218, 21, 199, 182, 31, 14, 186, 67, 85, 0, 4, 159, 116, 47, 238, 130, 221,
        164, 216, 168, 202, 39, 176,
    ];

    let issuer_canister_id =
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("wrong principal");
    let ii_canister_id =
        Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").expect("wrong principal");
    let issuer_origin = "https://metaissuer.vc/";
    let mut args = HashMap::new();
    args.insert("ageAtLeast".to_string(), ArgumentValue::Int(18));

    let req = ValidateVpRequest {
        vp_jwt: "eyJ0eXAiOiJKV1QiLCJhbGciOiJub25lIn0.eyJpc3MiOiJkaWQ6aWNwOm43ZTRnLTN2ZGZ0LXF3MnZ3LWt4Nnl0LXUyYmFsLTRzdG1tLW5vZDNiLTM1ejR6LXdzb2o3LW00azY2LWhxZSIsInZwIjp7IkBjb250ZXh0IjoiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiLCJ0eXBlIjoiVmVyaWZpYWJsZVByZXNlbnRhdGlvbiIsInZlcmlmaWFibGVDcmVkZW50aWFsIjpbImV5SnFkMnNpT25zaWEzUjVJam9pYjJOMElpd2lZV3huSWpvaVNXTkRjeUlzSW1zaU9pSk5SSGQzUkVGWlMwdDNXVUpDUVVkRWRVVk5Ra0ZuVFhOQlFXOUJRVUZCUVVGQlFVRkJRVVZDYTJ4dWNtSjBZMlI2YjJoeE1rNWxha1UwZVhjNVJ6UlNRVlZTZEZJeE1sZGpkbXByWVZKc1EydHlieUo5TENKcmFXUWlPaUprYVdRNmFXTndPbkozYkdkMExXbHBZV0ZoTFdGaFlXRmhMV0ZoWVdGaExXTmhhU0lzSW1Gc1p5STZJa2xqUTNNaWZRLmV5SmxlSEFpT2pFMk1qQXpNamsxTXpBc0ltbHpjeUk2SW1oMGRIQnpPaTh2YVdSbGJuUnBkSGt1YVdNd0xtRndjQzhpTENKdVltWWlPakUyTWpBek1qZzJNekFzSW1wMGFTSTZJbVJoZEdFNmRHVjRkQzl3YkdGcGJqdGphR0Z5YzJWMFBWVlVSaTA0TEhScGJXVnpkR0Z0Y0Y5dWN6b3hOakl3TXpJNE5qTXdNREF3TURBd01ERTFMR0ZzYVdGelgyaGhjMmc2TmpReVlUUmhaVEUyT0dFMU5qVTBZelJoT1dSaU56RXpZMlpoTTJZeE9USm1NVGRsTW1Jd1lqWXhaR1UxTTJOa1lXTmxaVEJsTjJJME9XUTVPVEppTnlJc0luTjFZaUk2SW1ScFpEcHBZM0E2YmpkbE5HY3RNM1prWm5RdGNYY3lkbmN0YTNnMmVYUXRkVEppWVd3dE5ITjBiVzB0Ym05a00ySXRNelY2TkhvdGQzTnZhamN0YlRSck5qWXRhSEZsSWl3aWRtTWlPbnNpUUdOdmJuUmxlSFFpT2lKb2RIUndjem92TDNkM2R5NTNNeTV2Y21jdk1qQXhPQzlqY21Wa1pXNTBhV0ZzY3k5Mk1TSXNJblI1Y0dVaU9sc2lWbVZ5YVdacFlXSnNaVU55WldSbGJuUnBZV3dpTENKSmJuUmxjbTVsZEVsa1pXNTBhWFI1U1dSQmJHbGhjeUpkTENKamNtVmtaVzUwYVdGc1UzVmlhbVZqZENJNmV5SkpiblJsY201bGRFbGtaVzUwYVhSNVNXUkJiR2xoY3lJNmV5Sm9ZWE5KWkVGc2FXRnpJam9pZEhkNGVXOHRjRFpuYVhNdFlYcHhOSEV0WkROME5UY3RkM3BsWmpZdFp6Wmphbm90ZWpaMWNYWXRiWFIwTjNrdFkydzNhM2N0WnpkclpHVXRkWEZsSW4xOWZYMC4yZG4zb210alpYSjBhV1pwWTJGMFpWa0JzZG5aOTZKa2RISmxaWU1CZ3dHREFZSUVXQ0FtZlJDTGdsRkp5SWtpZnlJTW9nRVRqT1lod3h4OFBQRjBFWDV3RlJFTHQ0TUNTR05oYm1semRHVnlnd0dEQVlNQ1NnQUFBQUFBQUFBQUFRR0RBWU1CZ3dKT1kyVnlkR2xtYVdWa1gyUmhkR0dDQTFnZ3pJWklfSHhUbHlRcVNwbjhIeG1vTUNDRVNWU2pRNnZLZU9ReHRmeUFLS21DQkZnZzd1dmNxV2E1VzVmT3pmWWs0QVYxaWhHMTBERzFkSUdGWmZxcWlmeGFKWXVDQkZnZ1NRTm1zQ3c3V2Y5b1g3XzRxUDBHZVRrSGQ4OEw2aEFneW5QRTR3U0x1SEtDQkZnZzFsdlY3UFlJLV9DR1RkNDBVcUxZbGlzb3A1X19LYVF0bXRrd09FTWlpVTJDQkZnZ2pyV2t4aTBwc2hPTkVOWWVLNnc2MXdldTA4SFNKa3k2MFV4TEFJdXFtNFdDQkZnZ0pvOGpFWXI2b2tGR2g3VzNwakJhRmI3YXN1RWFic3EtMUI2LXplTVBVVy1EQVlJRVdDRHF4cTdMc3dDcHJaQjdzcWU1WVhSdXB6ZTE5NDRaZkZXdy1leXptWFFaXzRNQ1JIUnBiV1dDQTBtUHVLN1UzWW1rdmhacGMybG5ibUYwZFhKbFdEQ3NGUHRJaEgteHZYQ3RSSXJMaEc0aHBfVFBBVEs1N0lvcmRDXzRYZm52Skc4NVQ4T2NVX2Z1c24wRWZDbjgtbjFrZEhKbFpZTUJnZ1JZSUQwTk1kYjh2QlpVTXBmRFRoZUtONTd1TTMyWkltVEJ6YTd4SjdUckQxVGtnd0pEYzJsbmd3SllJQ2oxek1JaXBXYmlja3VHV3dJeGhETWRVUE8yWE1ON1pfcGF3VmNncHUyOGd3R0NCRmdneUZkOFJZdjZHSG9lZVNCMGdSaEY1c3lIY1lzRmZTRUJGWlB2MEN2dnF4U0RBbGdnOGduN0RVLVRWbXEyc1VvM1RZWjZkNi1kSVZPd1VqZGI2VFhHYUtnVmpoQ0NBMEEiLCJleUpxZDJzaU9uc2lhM1I1SWpvaWIyTjBJaXdpWVd4bklqb2lTV05EY3lJc0ltc2lPaUpOUkhkM1JFRlpTMHQzV1VKQ1FVZEVkVVZOUWtGblRYTkJRVzlCUVVGQlFVRkJRVUZCVVVWQ05WVXlSMlF4VTJ0bFVIQk9MWEJQZDFsa1pGZENjelZuWDNwYWVGcFlZbmxLVFdKRU9GQnRTV2RYZHlKOUxDSnJhV1FpT2lKa2FXUTZhV053T25KeWEyRm9MV1p4WVdGaExXRmhZV0ZoTFdGaFlXRnhMV05oYVNJc0ltRnNaeUk2SWtsalEzTWlmUS5leUpsZUhBaU9qRTJNakF6TWprMU16QXNJbWx6Y3lJNkltaDBkSEJ6T2k4dmJXVjBZV2x6YzNWbGNpNTJZeThpTENKdVltWWlPakUyTWpBek1qZzJNekFzSW1wMGFTSTZJbVJoZEdFNmRHVjRkQzl3YkdGcGJqdGphR0Z5YzJWMFBWVlVSaTA0TEdsemMzVmxjanBvZEhSd2N6b3ZMMjFsZEdGcGMzTjFaWEl1ZG1Nc2RHbHRaWE4wWVcxd1gyNXpPakUyTWpBek1qZzJNekF3TURBd01EQXdNakFzYzNWaWFtVmpkRHAwZDNoNWJ5MXdObWRwY3kxaGVuRTBjUzFrTTNRMU55MTNlbVZtTmkxbk5tTnFlaTE2Tm5WeGRpMXRkSFEzZVMxamJEZHJkeTFuTjJ0a1pTMTFjV1VpTENKemRXSWlPaUprYVdRNmFXTndPblIzZUhsdkxYQTJaMmx6TFdGNmNUUnhMV1F6ZERVM0xYZDZaV1kyTFdjMlkycDZMWG8yZFhGMkxXMTBkRGQ1TFdOc04ydDNMV2MzYTJSbExYVnhaU0lzSW5aaklqcDdJa0JqYjI1MFpYaDBJam9pYUhSMGNITTZMeTkzZDNjdWR6TXViM0puTHpJd01UZ3ZZM0psWkdWdWRHbGhiSE12ZGpFaUxDSjBlWEJsSWpwYklsWmxjbWxtYVdGaWJHVkRjbVZrWlc1MGFXRnNJaXdpVm1WeWFXWnBaV1JCWjJVaVhTd2lZM0psWkdWdWRHbGhiRk4xWW1wbFkzUWlPbnNpVm1WeWFXWnBaV1JCWjJVaU9uc2lZV2RsUVhSTVpXRnpkQ0k2TVRoOWZYMTkuMmRuM29tdGpaWEowYVdacFkyRjBaVmtCc2RuWjk2SmtkSEpsWllNQmd3R0RBWUlFV0NBbWZSQ0xnbEZKeUlraWZ5SU1vZ0VUak9ZaHd4eDhQUEYwRVg1d0ZSRUx0NE1DU0dOaGJtbHpkR1Z5Z3dHREFZSUVXQ0FRaE0yZGJ3bXBZcWk4MmdoUUh0MVRkamtmMV9XTGdxZnFWbm5QRXNZRkVZTUNTZ0FBQUFBQUFBQUJBUUdEQVlNQmd3Sk9ZMlZ5ZEdsbWFXVmtYMlJoZEdHQ0ExZ2dkWkpuSmE0YnpZOUx1b3gyNVNFbmlSSGx2OE0tUnNudk9ONGRtYktPdmtpQ0JGZ2c3dXZjcVdhNVc1Zk96ZllrNEFWMWloRzEwREcxZElHRlpmcXFpZnhhSll1Q0JGZ2dNdUt6eVNxMVRPYmN4R1NTZFk4N05KbDF1MFNJUzFLZFpxNVZlU3hubG1TQ0JGZ2dqcldreGkwcHNoT05FTlllSzZ3NjF3ZXUwOEhTSmt5NjBVeExBSXVxbTRXQ0JGZ2d4VER0aHZWQzJQem80ZzRJOHpQVF9FdGs5MGJiMTMzTGszV0RTRWNZTzdlREFZSUVXQ0I3Nm1QNmdndXUwaXotMEo2aDZXNlNJeUVRNmo1a3luMmk5Y2doVnNmaXQ0TUNSSFJwYldXQ0EwbVV1SzdVM1lta3ZoWnBjMmxuYm1GMGRYSmxXRENYYWZnT1ZYNllzMWNCeXB0dEhoV1psWjhEei1KemFFbHNOU0ZEcllES0p1YW1HMmRFdlZ5SjZyTFhqVUFqNk5Wa2RISmxaWU1CZ2dSWUlHOTNoOEpUM1kwSHRIWHJ0TUVUS000Q2taOWxlR2NkVGxpcF80VWNNdk14Z3dKRGMybG5nd0pZSURtQzJfcUVicU9KOUpwNm4zU1JmYURYbjFZaDJPeTFkNjFiWFNvWk45Y1Bnd0pZSVBsbzEzS3RnNTFhdDN4RmNpaWVyTnh3WkRaY0k1NjlfMGxUTnZTU29nSWhnZ05BIl19fQ.".to_string(),
        effective_vc_subject: Principal::from_text("n7e4g-3vdft-qw2vw-kx6yt-u2bal-4stmm-nod3b-35z4z-wsoj7-m4k66-hqe").expect("wrong principal"),
        credential_spec: CredentialSpec { credential_type: "VerifiedAge".to_string(), arguments: Some(args) },
        issuer_origin: issuer_origin.to_string(),
        issuer_canister_id: Some(issuer_canister_id),
    };

    let rp_init = RpInit {
        ic_root_key_der,
        ii_vc_url: II_ISSUER_URL.to_string(),
        ii_canister_id,
        issuers: vec![IssuerData {
            vc_url: issuer_origin.to_string(),
            canister_id: issuer_canister_id,
        }],
        derivation_origin: "http://br5f7-7uaaa-aaaaa-qaaca-cai.localhost:4943/".to_string(),
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
