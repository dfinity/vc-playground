//! Tests related to general dapp management.

use canister_tests::api::http_request;
use canister_tests::framework::{env, principal_1, time};
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_response_verification::types::VerificationInfo;
use ic_response_verification::verify_request_response_pair;
use ic_test_state_machine_client::{CallError, StateMachine};
use internet_identity_interface::http_gateway::{HttpRequest, HttpResponse};

use relying_party::rp_api::ContentData;
use serde_bytes::ByteBuf;
use std::time::Duration;

#[allow(dead_code)]
mod util;
use crate::util::{
    do_add_exclusive_content, do_list_exclusive_content, do_list_images, install_rp,
    RELYING_PARTY_WASM,
};

/// Verifies that the expected assets is delivered and certified.
#[test]
fn issuer_canister_serves_http_assets() -> Result<(), CallError> {
    fn verify_response_certification(
        env: &StateMachine,
        canister_id: CanisterId,
        request: HttpRequest,
        http_response: HttpResponse,
        min_certification_version: u16,
    ) -> VerificationInfo {
        verify_request_response_pair(
            ic_http_certification::HttpRequest {
                method: request.method,
                url: request.url,
                headers: request.headers,
                body: request.body.into_vec(),
            },
            ic_http_certification::HttpResponse {
                status_code: http_response.status_code,
                headers: http_response.headers,
                body: http_response.body.into_vec(),
            },
            canister_id.as_slice(),
            time(env) as u128,
            Duration::from_secs(300).as_nanos(),
            &env.root_key(),
            min_certification_version as u8,
        )
        .unwrap_or_else(|e| panic!("validation failed: {e}"))
    }

    let env = env();
    let canister_id = install_rp(&env, None);

    // for each asset and certification version, fetch the asset, check the HTTP status code, headers and certificate.
    for certification_version in 1..=2 {
        for image_name in [
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
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/images/{}", image_name),
                headers: vec![],
                body: ByteBuf::new(),
                certificate_version: Some(certification_version),
            };
            let http_response = http_request(&env, canister_id, &request)?;
            assert_eq!(http_response.status_code, 200);

            let result = verify_response_certification(
                &env,
                canister_id,
                request,
                http_response,
                certification_version,
            );
            assert_eq!(result.verification_version, certification_version);
        }

        // For non-existing assets, 404 should be returned.
        for image_name in ["DoesNotExist.jpg", "AnotherMissing.bmp"] {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/images/{}", image_name),
                headers: vec![],
                body: ByteBuf::new(),
                certificate_version: Some(certification_version),
            };
            let http_response = http_request(&env, canister_id, &request)?;
            assert_eq!(http_response.status_code, 404);
        }
    }

    Ok(())
}

#[test]
fn should_upgrade_relying_party() -> Result<(), CallError> {
    let env = env();
    let canister_id = install_rp(&env, None);

    // Verify the canister is running.
    let list = do_list_images(&env, canister_id);
    assert_eq!(list.images.len(), 11);

    // Upgrade the canister
    let arg = candid::encode_one("()").expect("error encoding issuer init arg as candid");
    env.upgrade_canister(canister_id, RELYING_PARTY_WASM.clone(), arg, None)?;

    // Verify the canister is still running.
    let list = do_list_images(&env, canister_id);
    assert_eq!(list.images.len(), 11);
    Ok(())
}

#[test]
fn should_retain_data_after_upgrade() -> Result<(), CallError> {
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
    let content_list = do_list_exclusive_content(&env, None, canister_id);
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);

    // Upgrade the canister.
    let arg = candid::encode_one("()").expect("error encoding issuer init arg as candid");
    env.upgrade_canister(canister_id, RELYING_PARTY_WASM.clone(), arg, None)?;

    // Check that canister still works, and that it has the data from before the upgrade.
    let content_list = do_list_exclusive_content(&env, None, canister_id);
    assert_eq!(content_list.content_items.len(), 1);
    assert_eq!(content_list.content_items[0], expected_content_data);

    let images_list = do_list_images(&env, canister_id);
    assert_eq!(images_list.images.len(), 11);
    Ok(())
}
