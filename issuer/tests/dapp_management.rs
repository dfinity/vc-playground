//! Tests related to general dapp management.

use canister_tests::api::http_request;
use canister_tests::framework::{env, principal_1, time};
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_response_verification::types::VerificationInfo;
use ic_response_verification::verify_request_response_pair;
use ic_test_state_machine_client::{CallError, StateMachine};
use internet_identity_interface::http_gateway::{HttpRequest, HttpResponse};

use serde_bytes::ByteBuf;
use std::time::Duration;

#[allow(dead_code)]
mod util;
use crate::util::{
    add_group_with_member, api, do_add_group, do_get_group, install_issuer, DUMMY_ISSUER_INIT,
    META_ISSUER_WASM,
};

#[test]
fn should_configure() {
    let env = env();
    let issuer_id = install_issuer(&env, None);
    api::configure(&env, issuer_id, &DUMMY_ISSUER_INIT).expect("API call failed");
}

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
    let canister_id = install_issuer(&env, None);

    // for each asset and certification version, fetch the asset, check the HTTP status code, headers and certificate.

    for certification_version in 1..=2 {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/".to_string(),
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

    Ok(())
}

#[test]
fn should_upgrade_issuer() -> Result<(), CallError> {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let arg = candid::encode_one("()").expect("error encoding issuer init arg as candid");
    let group_name = "Some group";
    let group_data_before = do_add_group(group_name, principal_1(), &env, issuer_id);
    env.upgrade_canister(issuer_id, META_ISSUER_WASM.clone(), arg, None)?;

    // Verify the canister is still running.
    let group_data_after = do_get_group(group_name, principal_1(), &env, issuer_id);
    assert_eq!(group_data_before, group_data_after);
    Ok(())
}

#[test]
fn should_retain_groups_upgrade() -> Result<(), CallError> {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    // Create a group, and record its data.
    let authorized_principal = principal_1();
    let group_name = "Some group";
    add_group_with_member(
        group_name,
        authorized_principal,
        "Alice",
        authorized_principal,
        &env,
        issuer_id,
    );
    let group_data_before = do_get_group(group_name, authorized_principal, &env, issuer_id);

    // Upgrade the canister.
    let arg = candid::encode_one("()").expect("error encoding issuer init arg as candid");
    env.upgrade_canister(issuer_id, META_ISSUER_WASM.clone(), arg, None)?;
    env.advance_time(std::time::Duration::from_secs(2));

    // Check that canister still works, and that it has the data from before the upgrade.
    let other_group_data = do_add_group("Another group", authorized_principal, &env, issuer_id);
    assert_ne!(
        group_data_before.stats.created_timestamp_ns,
        other_group_data.stats.created_timestamp_ns
    );
    let group_data_after = do_get_group(group_name, authorized_principal, &env, issuer_id);
    assert_eq!(group_data_before, group_data_after);
    Ok(())
}
