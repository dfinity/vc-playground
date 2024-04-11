//! Tests related to Verifiable Credentials API.

use assert_matches::assert_matches;
use candid::Principal;
use canister_sig_util::{extract_raw_root_pk_from_der, CanisterSigPublicKey};
use canister_tests::api::internet_identity::vc_mvp as ii_api;
use canister_tests::flows;
use canister_tests::framework::{env, principal_1, test_principal};

use ic_cdk::api::management_canister::main::CanisterId;
use ic_test_state_machine_client::{call_candid_as, CallError, StateMachine};
use internet_identity_interface::internet_identity::types::vc_mvp::{
    GetIdAliasRequest, PrepareIdAliasRequest,
};
use internet_identity_interface::internet_identity::types::FrontendHostname;
use relying_party::rp_api;
use relying_party::rp_api::{
    AddExclusiveContentRequest, ContentData, ContentError, IssuerData, ValidateVpRequest,
};
use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use vc_util::issuer_api::{
    ArgumentValue, CredentialSpec, GetCredentialRequest, Icrc21ConsentPreferences, Icrc21Error,
    Icrc21VcConsentMessageRequest, IssueCredentialError, PrepareCredentialRequest,
    SignedIdAlias as SignedIssuerIdAlias,
};
use vc_util::{
    build_ii_verifiable_presentation_jwt, get_verified_id_alias_from_jws,
    validate_claims_match_spec, verify_credential_jws_with_canister_id,
};

#[allow(dead_code)]
mod util;
use crate::util::{
    add_group_with_member, api, install_canister, install_issuer, IssuerInit,
    DUMMY_ALIAS_ID_DAPP_PRINCIPAL, DUMMY_ISSUER_INIT, DUMMY_SIGNED_ID_ALIAS, II_WASM,
    RELYING_PARTY_WASM,
};

const DUMMY_GROUP_NAME: &str = "Dummy group";

#[test]
fn should_get_vc_consent_message() {
    let env = env();
    let canister_id = install_issuer(&env, None);

    let consent_message_request = Icrc21VcConsentMessageRequest {
        credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
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

fn verified_member_credential_spec(group_name: &str, owner: Principal) -> CredentialSpec {
    let mut args = HashMap::new();
    args.insert(
        "groupName".to_string(),
        ArgumentValue::String(group_name.to_string()),
    );
    args.insert("owner".to_string(), ArgumentValue::String(owner.to_text()));
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Err(e) if format!("{:?}", e).contains("is not a member of group"));
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
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
    let owner = principal_1();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        owner,
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, owner),
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, owner),
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
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
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, principal_1()),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Err(IssueCredentialError::InvalidIdAlias(_)));
}

#[test]
fn should_prepare_verfied_member_credential_for_authorized_principal() {
    let env = env();
    let issuer_id = install_issuer(&env, Some(DUMMY_ISSUER_INIT.clone()));
    let authorized_principal = Principal::from_text(DUMMY_ALIAS_ID_DAPP_PRINCIPAL).unwrap();
    let owner = principal_1();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        owner,
        authorized_principal,
        &env,
        issuer_id,
    );
    let response = api::prepare_credential(
        &env,
        issuer_id,
        authorized_principal,
        &PrepareCredentialRequest {
            credential_spec: verified_member_credential_spec(DUMMY_GROUP_NAME, owner),
            signed_id_alias: DUMMY_SIGNED_ID_ALIAS.clone(),
        },
    )
    .expect("API call failed");
    assert_matches!(response, Ok(_));
}

fn rp_add_exclusive_content(
    env: &StateMachine,
    canister_id: CanisterId,
    sender: Principal,
    req: AddExclusiveContentRequest,
) -> Result<Result<ContentData, ContentError>, CallError> {
    call_candid_as(env, canister_id, sender, "add_exclusive_content", (req,)).map(|(x,)| x)
}

fn rp_validate_ii_vp(
    env: &StateMachine,
    canister_id: CanisterId,
    sender: Principal,
    req: ValidateVpRequest,
) -> Result<Result<(), ContentError>, CallError> {
    call_candid_as(env, canister_id, sender, "validate_ii_vp", (req,)).map(|(x,)| x)
}

/// Verifies that credentials are being created including II interactions.
#[test]
fn should_issue_share_and_validate_e2e() -> Result<(), CallError> {
    let env = env();
    let ii_url = FrontendHostname::from(vc_util::II_ISSUER_URL);
    let issuer_url = FrontendHostname::from("https://metaissuer.vc/");
    let rp_url = FrontendHostname::from("https://some-dapp.com/");

    // Setup canisters
    let ii_id = install_canister::<IssuerInit>(&env, II_WASM.clone(), None);
    let issuer_id = install_issuer(
        &env,
        Some(IssuerInit {
            ic_root_key_der: env.root_key().to_vec(),
            idp_canister_ids: vec![ii_id],
            ..DUMMY_ISSUER_INIT.clone()
        }),
    );
    let rp_id = install_canister(
        &env,
        RELYING_PARTY_WASM.clone(),
        Some(rp_api::RpInit {
            ic_root_key_der: env.root_key().to_vec(),
            ii_origin: ii_url.clone(),
            ii_canister_id: ii_id,
            issuers: vec![IssuerData {
                origin: issuer_url.clone(),
                canister_id: issuer_id,
            }],
        }),
    );

    // Register a user with II
    let identity_number = flows::register_anchor(&env, ii_id);

    let prepare_id_alias_req = PrepareIdAliasRequest {
        identity_number,
        relying_party: rp_url.clone(),
        issuer: issuer_url.clone(),
    };

    let prepared_id_alias =
        ii_api::prepare_id_alias(&env, ii_id, principal_1(), prepare_id_alias_req)?
            .expect("prepare id_alias failed");

    let canister_sig_pk =
        CanisterSigPublicKey::try_from(prepared_id_alias.canister_sig_pk_der.as_ref())
            .expect("failed parsing canister sig pk");

    let get_id_alias_req = GetIdAliasRequest {
        identity_number,
        relying_party: rp_url,
        issuer: issuer_url.clone(),
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

    // Add a group to the meta issuer with the expected member.
    let authorized_principal = alias_tuple.id_dapp;
    let owner = principal_1();
    add_group_with_member(
        DUMMY_GROUP_NAME,
        owner,
        authorized_principal,
        &env,
        issuer_id,
    );

    let credential_spec = verified_member_credential_spec(DUMMY_GROUP_NAME, owner);

    // Add an exclusive content to the rp, gated by a VC.
    let content_url = "http://example.com";

    rp_add_exclusive_content(
        &env,
        rp_id,
        principal_1(),
        AddExclusiveContentRequest {
            content_name: "restricted content".to_string(),
            url: content_url.to_string(),
            credential_group_name: DUMMY_GROUP_NAME.to_string(),
            credential_group_owner: owner,
        },
    )
    .expect("API call failed")
    .expect("Failed add_exclusive_content");

    // Request the credential.
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
    let requested_vc_jws = get_credential_response
        .expect("failed get_credential")
        .vc_jws;
    let claims = verify_credential_jws_with_canister_id(
        &requested_vc_jws,
        &issuer_id,
        &root_pk_raw,
        env.time().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
    )
    .expect("credential verification failed");
    let vc_claims = claims.vc().expect("missing VC claims");
    validate_claims_match_spec(vc_claims, &credential_spec).expect("Clam validation failed");
    // Request credential validation from RP's backend.
    let vp_jwt = build_ii_verifiable_presentation_jwt(
        id_alias_credentials.rp_id_alias_credential.id_dapp,
        id_alias_credentials.rp_id_alias_credential.credential_jws,
        requested_vc_jws,
    )
    .expect("failed building VP");
    let validate_vp_request = ValidateVpRequest {
        vp_jwt,
        effective_vc_subject: id_alias_credentials.rp_id_alias_credential.id_dapp,
        credential_spec,
        issuer_origin: issuer_url.to_string(),
        issuer_canister_id: Some(issuer_id),
    };
    let result = rp_validate_ii_vp(&env, rp_id, principal_1(), validate_vp_request)?;
    assert!(result.is_ok(), "{:?}", result);
    Ok(())
}
