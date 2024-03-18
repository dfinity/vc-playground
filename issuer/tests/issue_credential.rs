//! Tests related to Verifiable Credentials API.

use assert_matches::assert_matches;
use candid::Principal;
use canister_sig_util::{extract_raw_root_pk_from_der, CanisterSigPublicKey};
use canister_tests::api::http_request;
use canister_tests::api::internet_identity::vc_mvp as ii_api;
use canister_tests::flows;
use canister_tests::framework::{env, principal_1, test_principal, time};
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_response_verification::types::VerificationInfo;
use ic_response_verification::verify_request_response_pair;
use ic_test_state_machine_client::{CallError, StateMachine};
use internet_identity_interface::http_gateway::{HttpRequest, HttpResponse};
use internet_identity_interface::internet_identity::types::vc_mvp::{
    GetIdAliasRequest, PrepareIdAliasRequest,
};
use internet_identity_interface::internet_identity::types::FrontendHostname;
use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};
use vc_util::issuer_api::{
    ArgumentValue, CredentialSpec, GetCredentialRequest, Icrc21ConsentPreferences, Icrc21Error,
    Icrc21VcConsentMessageRequest, IssueCredentialError, PrepareCredentialRequest,
    SignedIdAlias as SignedIssuerIdAlias,
};
use vc_util::{
    get_verified_id_alias_from_jws, validate_claims_match_spec,
    verify_credential_jws_with_canister_id,
};

#[allow(dead_code)]
mod util;
use crate::util::{add_group_with_member, META_ISSUER_WASM};
use util::{
    api, do_add_group, do_get_group, install_canister, install_issuer, IssuerInit,
    DUMMY_ALIAS_ID_DAPP_PRINCIPAL, DUMMY_ISSUER_INIT, DUMMY_SIGNED_ID_ALIAS, II_WASM,
};

const DUMMY_GROUP_NAME: &str = "Dummy group";

#[test]
fn should_get_vc_consent_message() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
        preferences: Icrc21ConsentPreferences {
            language: "en-US".to_string(),
        },
    };

    let consent_info =
        api::vc_consent_message(&env, canister_id, principal_1(), &consent_message_request)
            .expect("API call failed")
            .expect("Failed to obtain consent info");
    assert!(consent_info.consent_message.contains("Verified Member"));
}

#[test]
fn should_fail_vc_consent_message_if_not_supported() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: CredentialSpec {
            credential_type: "VerifiedResident".to_string(),
            arguments: None,
        },
        preferences: Icrc21ConsentPreferences {
            language: "en-US".to_string(),
        },
    };

    let response =
        api::vc_consent_message(&env, canister_id, principal_1(), &consent_message_request)
            .expect("API call failed");
    assert_matches!(response, Err(Icrc21Error::ConsentMessageUnavailable(_)));
}

#[test]
fn should_fail_vc_consent_message_if_missing_arguments() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: CredentialSpec {
            credential_type: "VerifiedEmployee".to_string(),
            arguments: None,
        },
        preferences: Icrc21ConsentPreferences {
            language: "en-US".to_string(),
        },
    };

    let response =
        api::vc_consent_message(&env, canister_id, principal_1(), &consent_message_request)
            .expect("API call failed");
    assert_matches!(response, Err(Icrc21Error::ConsentMessageUnavailable(_)));
}

#[test]
fn should_fail_vc_consent_message_if_missing_required_argument() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let mut args = HashMap::new();
    args.insert("wrongArgument".to_string(), ArgumentValue::Int(42));

    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: CredentialSpec {
            credential_type: "VerifiedEmployee".to_string(),
            arguments: None,
        },
        preferences: Icrc21ConsentPreferences {
            language: "en-US".to_string(),
        },
    };

    let response =
        api::vc_consent_message(&env, canister_id, principal_1(), &consent_message_request)
            .expect("API call failed");
    assert_matches!(response, Err(Icrc21Error::ConsentMessageUnavailable(_)));
}

fn verified_member_credential_spec(group_name: &str) -> CredentialSpec {
    let mut args = HashMap::new();
    args.insert(
        "groupName".to_string(),
        ArgumentValue::String(group_name.to_string()),
    );
    CredentialSpec {
        credential_type: "VerifiedMember".to_string(),
        arguments: Some(args),
    }
}

#[test]
fn should_fail_prepare_credential_for_unauthorized_principal() {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let response = api::prepare_credential(
        &env,
        issuer_id,
        Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap(),
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Err(e) if format!("{:?}", e).contains("not an accepted member"));
}

#[test]
fn should_fail_prepare_credential_for_wrong_sender() {
    let env = env();
    let issuer_id = install_issuer(&env, None);
    let signed_id_alias = DUMMY_SIGNED_ID_ALIAS.clone();

    let response = api::prepare_credential(
        &env,
        issuer_id,
        principal_1(), // not the same as contained in signed_id_alias
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias,
        },
    )
    .expect("API call failed");
    assert_matches!(response,
        Err(IssueCredentialError::InvalidIdAlias(e)) if e.contains("id alias could not be verified")
    );
}

#[test]
fn should_fail_get_credential_for_wrong_sender() {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let signed_id_alias = DUMMY_SIGNED_ID_ALIAS.clone();
    let authorized_principal = Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        authorized_principal,
        "Alice",
        authorized_principal,
        &env,
        issuer_id,
    );
    let unauthorized_principal = test_principal(2);

    let prepare_credential_response = api::prepare_credential(
        &env,
        issuer_id,
        authorized_principal,
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: signed_id_alias.clone(),
        },
    )
    .expect("API call failed")
    .expect("failed to prepare credential");

    let get_credential_response = api::get_credential(
        &env,
        issuer_id,
        unauthorized_principal,
        &GetCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias,
            prepared_context: prepare_credential_response.prepared_context,
        },
    )
    .expect("API call failed");
    assert_matches!(get_credential_response,
        Err(IssueCredentialError::InvalidIdAlias(e)) if e.contains("id alias could not be verified")
    );
}

#[test]
fn should_fail_prepare_credential_for_anonymous_caller() {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let response = api::prepare_credential(
        &env,
        issuer_id,
        Principal::anonymous(),
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response,
        Err(IssueCredentialError::InvalidIdAlias(e)) if e.contains("id alias could not be verified")
    );
}

#[test]
fn should_fail_prepare_credential_for_wrong_root_key() {
    let env = env();
    let issuer_id = install_issuer(
        &env,
        Some(IssuerInit {
            ic_root_key_der: canister_sig_util::IC_ROOT_PK_DER.to_vec(), // does not match the DUMMY_ROOT_KEY, which is used in DUMMY_ALIAS_JWS
            ..DUMMY_ISSUER_INIT.clone()
        }),
    );
    let response = api::prepare_credential(
        &env,
        issuer_id,
        Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap(),
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Err(IssueCredentialError::InvalidIdAlias(_)));
}

#[test]
fn should_fail_prepare_credential_for_wrong_idp_canister_id() {
    let env = env();
    let issuer_id = install_issuer(
        &env,
        Some(IssuerInit {
            idp_canister_ids: vec![Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()], // does not match the DUMMY_II_CANISTER_ID, which is used in DUMMY_ALIAS_JWS
            ..DUMMY_ISSUER_INIT.clone()
        }),
    );
    let response = api::prepare_credential(
        &env,
        issuer_id,
        Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap(),
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Err(IssueCredentialError::InvalidIdAlias(_)));
}

#[test]
fn should_prepare_early_adopter_credential_for_authorized_principal() {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let authorized_principal = Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        authorized_principal,
        "Alice",
        authorized_principal,
        &env,
        issuer_id,
    );
    let response = api::prepare_credential(
        &env,
        issuer_id,
        authorized_principal,
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Ok(_));
}

/// Verifies that credentials are being created including II interactions.
#[test]
fn should_issue_credential_e2e() -> Result<(), CallError> {
    let env = env();
    let ii_id = install_canister(&env, II_WASM.clone());
    let issuer_id = install_issuer(
        &env,
        Some(IssuerInit {
            ic_root_key_der: env.root_key().to_vec(),
            idp_canister_ids: vec![ii_id],
            ..DUMMY_ISSUER_INIT.clone()
        }),
    );
    let identity_number = flows::register_anchor(&env, ii_id);
    let relying_party = FrontendHostname::from("https://some-dapp.com");
    let issuer = FrontendHostname::from("https://some-issuer.com");

    let prepare_id_alias_req = PrepareIdAliasRequest {
        identity_number,
        relying_party: relying_party.clone(),
        issuer: issuer.clone(),
    };

    let prepared_id_alias =
        ii_api::prepare_id_alias(&env, ii_id, principal_1(), prepare_id_alias_req)?
            .expect("prepare id_alias failed");

    let canister_sig_pk =
        CanisterSigPublicKey::try_from(prepared_id_alias.canister_sig_pk_der.as_ref())
            .expect("failed parsing canister sig pk");

    let get_id_alias_req = GetIdAliasRequest {
        identity_number,
        relying_party,
        issuer,
        rp_id_alias_jwt: prepared_id_alias.rp_id_alias_jwt,
        issuer_id_alias_jwt: prepared_id_alias.issuer_id_alias_jwt,
    };
    let id_alias_credentials = ii_api::get_id_alias(&env, ii_id, principal_1(), get_id_alias_req)?
        .expect("get id_alias failed");

    let root_pk_raw =
        extract_raw_root_pk_from_der(&env.root_key()).expect("Failed decoding IC root key.");
    let alias_tuple = get_verified_id_alias_from_jws(
        &id_alias_credentials
            .issuer_id_alias_credential
            .credential_jws,
        &id_alias_credentials.issuer_id_alias_credential.id_dapp,
        &canister_sig_pk.canister_id,
        &root_pk_raw,
        env.time().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
    )
    .expect("Invalid ID alias");

    let authorized_principal = alias_tuple.id_dapp;
    add_group_with_member(
        DUMMY_GROUP_NAME,
        authorized_principal,
        "Alice",
        authorized_principal,
        &env,
        issuer_id,
    );

    let credential_spec = verified_member_credential_spec(DUMMY_GROUP_NAME);
    let prepared_credential = api::prepare_credential(
        &env,
        issuer_id,
        alias_tuple.id_dapp,
        &PrepareCredentialRequest {
            credential_spec: credential_spec.clone(),
            signed_id_alias: SignedIssuerIdAlias {
                credential_jws: id_alias_credentials
                    .issuer_id_alias_credential
                    .credential_jws
                    .clone(),
            },
        },
    )?
    .expect("failed to prepare credential");

    let get_credential_response = api::get_credential(
        &env,
        issuer_id,
        alias_tuple.id_dapp,
        &GetCredentialRequest {
            credential_spec: credential_spec.clone(),
            signed_id_alias: SignedIssuerIdAlias {
                credential_jws: id_alias_credentials
                    .issuer_id_alias_credential
                    .credential_jws
                    .clone(),
            },
            prepared_context: prepared_credential.prepared_context,
        },
    )?;
    let claims = verify_credential_jws_with_canister_id(
        &get_credential_response.unwrap().vc_jws,
        &issuer_id,
        &root_pk_raw,
        env.time().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
    )
    .expect("credential verification failed");
    let vc_claims = claims.vc().expect("missing VC claims");
    validate_claims_match_spec(vc_claims, &credential_spec).expect("Clam validation failed");

    Ok(())
}

#[test]
fn should_configure() {
    let env = env();
    let issuer_id = install_issuer(&env, None);
    api::configure(&env, issuer_id, &DUMMY_ISSUER_INIT).expect("API call failed");
}

/// Verifies that the expected assets is delivered and certified.
#[test]
#[ignore]
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
    env.upgrade_canister(issuer_id, META_ISSUER_WASM.clone(), arg, None)?;

    // Verify the canister is still running.
    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME),
        preferences: Icrc21ConsentPreferences {
            language: "en-US".to_string(),
        },
    };
    let _ = api::vc_consent_message(&env, issuer_id, principal_1(), &consent_message_request)
        .expect("API call failed")
        .expect("Failed to obtain consent info");
    Ok(())
}

#[test]
fn should_retain_groups_upgrade() -> Result<(), CallError> {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    // Create a group, and record its data.
    let authorized_principal = principal_1();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        authorized_principal,
        "Alice",
        authorized_principal,
        &env,
        issuer_id,
    );
    let group_data_before = do_get_group(DUMMY_GROUP_NAME, authorized_principal, &env, issuer_id);

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
    let group_data_after = do_get_group(DUMMY_GROUP_NAME, authorized_principal, &env, issuer_id);
    assert_eq!(group_data_before, group_data_after);
    Ok(())
}
