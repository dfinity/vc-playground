use candid::{CandidType, Deserialize, Principal};
use canister_tests::framework::get_wasm_path;
use ic_canister_sig_creation::IC_ROOT_PK_DER;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_test_state_machine_client::{
    call_candid, call_candid_as, query_candid_as, CallError, StateMachine,
};
use ic_verifiable_credentials::issuer_api::{
    ArgumentValue, GetCredentialRequest, Icrc21ConsentInfo, Icrc21Error,
    Icrc21VcConsentMessageRequest, IssueCredentialError, IssuedCredentialData,
    PrepareCredentialRequest, PreparedCredentialData, SignedIdAlias as SignedIssuerIdAlias,
};
use lazy_static::lazy_static;
use meta_issuer::groups_api::{
    AddGroupRequest, FullGroupData, GetGroupRequest, GroupTypes, GroupsError, JoinGroupRequest,
    ListGroupsRequest, MembershipStatus, MembershipUpdate, PublicGroupsData, SetUserRequest,
    UpdateMembershipRequest, UserData,
};
use std::collections::HashMap;
use std::path::PathBuf;

pub const DUMMY_II_CANISTER_ID: &str = "fgte5-ciaaa-aaaad-aaatq-cai";

/// Created with the Dummy Relying Party: `https://l7rua-raaaa-aaaap-ahh6a-cai.icp0.io/`/
/// Dummy alias JWS for testing, valid wrt `DUMMY_` variables
/// id dapp: 7eboi-tyuys-aqm4c-w2l7i-vgucm-xvawx-lemzx-6kq2g-f53u7-yvfh2-nae
/// id alias: evacf-r7slg-yca2r-i353q-x2lyx-jvb77-hh6ci-fwvuv-ckqis-plqzg-zqe
/// Uses mainnet root key.
const DUMMY_ALIAS_JWS: &str ="eyJqd2siOnsia3R5Ijoib2N0IiwiYWxnIjoiSWNDcyIsImsiOiJNRHd3REFZS0t3WUJCQUdEdUVNQkFnTXNBQW9BQUFBQUFHQUFKd0VCTTc1TzhhaEdaUm5pQ3lPWHZBcnlKaWNXaWxEbmVmdy1ZaW9mU1FScDF4USJ9LCJraWQiOiJkaWQ6aWNwOmZndGU1LWNpYWFhLWFhYWFkLWFhYXRxLWNhaSIsImFsZyI6IkljQ3MifQ.eyJleHAiOjE3MzA3OTcyMzMsImlzcyI6Imh0dHBzOi8vaWRlbnRpdHkuaWMwLmFwcC8iLCJuYmYiOjE3MzA3OTYzMzMsImp0aSI6ImRhdGE6dGV4dC9wbGFpbjtjaGFyc2V0PVVURi04LHRpbWVzdGFtcF9uczoxNzMwNzk2MzMzNjcyMjQ1NDc0LGFsaWFzX2hhc2g6Yjc3OTVmZDJiNDY3MTA2NGE2YjZmNmFlMDIwZjk3ZWJlYmE2NWI2ZmUxNTA0ZTIxZDcyOGJhZTAxMzQwMjIwYyIsInN1YiI6ImRpZDppY3A6N2Vib2ktdHl1eXMtYXFtNGMtdzJsN2ktdmd1Y20teHZhd3gtbGVtengtNmtxMmctZjUzdTcteXZmaDItbmFlIiwidmMiOnsiQGNvbnRleHQiOiJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJJbnRlcm5ldElkZW50aXR5SWRBbGlhcyJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJJbnRlcm5ldElkZW50aXR5SWRBbGlhcyI6eyJkZXJpdmF0aW9uT3JpZ2luIjoiaHR0cHM6Ly9sN3J1YS1yYWFhYS1hYWFhcC1haGg2YS1jYWkuaWMwLmFwcCIsImhhc0lkQWxpYXMiOiJldmFjZi1yN3NsZy15Y2Eyci1pMzUzcS14Mmx5eC1qdmI3Ny1oaDZjaS1md3Z1di1ja3Fpcy1wbHF6Zy16cWUifX19fQ.2dn3omtjZXJ0aWZpY2F0ZVkFbdnZ96NkdHJlZYMBgwGDAYIEWCCmWvo44iQiiGOonrtzz-Vc-cC15sj4dWw5iM14OXZei4MCSGNhbmlzdGVygwGDAYMBgwGDAYMBggRYIDYj_Sp7TSeDkLSw4CZEmywdGY3WFsm-z6vpj5khJtyYgwGCBFggQEL7KETbIG4XJKJI7vOT9csdIigPKY2Uj8GOCkCFM0ODAYIEWCCNPbxbGsgH608xO5FxLblP30pQBoIHcZ8cujd3GyrI74MCSgAAAAAAYAAnAQGDAYMBgwJOY2VydGlmaWVkX2RhdGGCA1gghQUiyD3srYn8hjdcO3nMj7jlr2iCzEEYYSOE5AGaLcaCBFggbM1rsxpUdh1KVunP2MujhNW4-0cYToyhPLcOBPIgms6CBFggFFnK2JDvqtu-zxXcTm9dS3KmrRTa0b8rl24oBq8E34aCBFggPeeB3ggR9ahGkWbFlPlDPZZvaG9PQGWtk5XjC_rBU-KCBFggyyqUBXAErjNvtSujkRfPkKqt7-At3-kgW8wTyPYVCgKCBFggvB-bTFT2brj8JTgekGQa5Z74fFkBhjVRYqUstIdSQsuCBFggDO2s0avVup1cZkxYQ6qVKrLqXxr8XPudpvDG9iA1GUuCBFggdHlZ016ceBDpr5fsJ5C0-e4IJ9HeJ2BXrLo-Dd-Y1L6CBFggTfWPC5xMuHTrfpld1Pu_3QbKRFkwtChcdf9_m0N8M1aDAYIEWCB4M9cRIc5igAWtqNuOK58ubw19qHxKZ4IKmvTgYBgWxoMCRHRpbWWCA0mMx__KicPBghhpc2lnbmF0dXJlWDCXecrOkjZ2CfdPluCeq6y0YySMAVru6vpGX8GYOryRGQeQJn18wtzZ5p7agOfdEc9qZGVsZWdhdGlvbqJpc3VibmV0X2lkWB0sVbNH7PJobIN4HWxZ0bQ-e0y6jetsGzdhB_LNAmtjZXJ0aWZpY2F0ZVkClNnZ96JkdHJlZYMBggRYIG27-yoKqqCgQXw-1FRQ4P6eqesH5QLoYdx25aRYKNRYgwGDAYIEWCCT5wenc4PnWnz1HXMCJVfV3hiQZXTbD5LE9buS_bTEB4MCRnN1Ym5ldIMBgwGDAYMBggRYIIc5-77dPe2qj-9BhwNnwJBb3jdrY9034rF2-wi1ggUvgwGCBFggg_I0Z_VeOyicnsbKOf6cAu9lvR5u1cEYZsKqvna-qnaDAYMCWB0sVbNH7PJobIN4HWxZ0bQ-e0y6jetsGzdhB_LNAoMBgwJPY2FuaXN0ZXJfcmFuZ2VzggNYMtnZ94KCSgAAAAAAYAAAAQFKAAAAAABgAK4BAYJKAAAAAABgALABAUoAAAAAAG___wEBgwJKcHVibGljX2tleYIDWIUwgYIwHQYNKwYBBAGC3HwFAwECAQYMKwYBBAGC3HwFAwIBA2EAkAdRIHeOshpTCgK8x2Pn9KGSkzUGlmr3tUwQpNKyTeaoayAONEC65iZ79MSI2aEdBHLDjBtiIRmPmOTmiCujilpOOqWvzombf4Je2VrfoSYpaIBzVW8nR1JyE-jXPkDOggRYIDbzzSV9kPs45CWX8ZOl4DHb1YW2KSeTuwTbR5SAPOBuggRYIIj-oNtp84-c8_uoj4oEDzytya53cvoaQGpupGT6hYueggRYIGlh7xN8Ku4LBGcILvbTwSwD6TATtgKky2IUJw5ISGPxggRYILmsedpaOOJBqlW6S4s2vPkoERAwZyW5emA-8pAW1P7hgwJEdGltZYIDScHNhcbntLSCGGlzaWduYXR1cmVYMIswpM8elThfCHQknyufP2s1-hgDV1L1LaepAY-bI_m2DjAMYnsakUpuF9aH7158x2R0cmVlgwGCBFggwjkSumdG4A6WIIzuVt7a82HtlqvZR4sQoeY-2G7V8_mDAkNzaWeDAlgggcprggWNF0Gv498eoWFpgAVS_vb_N-s4Fju-4gqFfnGDAYIEWCD-YbyK4DAm8dqHqxfaDyXATsr5w23cQCVc8EOWLFP9U4MCWCDaXyxpJhYi2JuSYjrNGWPbypHJ8_RUoMpPPXz6ursns4IDQA";
pub const DUMMY_ALIAS_ID_DAPP_PRINCIPAL: &str =
    "7eboi-tyuys-aqm4c-w2l7i-vgucm-xvawx-lemzx-6kq2g-f53u7-yvfh2-nae";

pub const DUMMY_ISSUER_DERIVATION_ORIGIN: &str = "https://l7rua-raaaa-aaaap-ahh6a-cai.ic0.app";
pub const DUMMY_ISSUER_FRONTEND_HOSTNAME: &str = "https://l7rua-raaaa-aaaap-ahh6a-cai.ic0.app";

lazy_static! {
    /// Gzipped Wasm module for the current VC Playground Meta-Issuer build, i.e. the one we're testing
    pub static ref META_ISSUER_WASM: Vec<u8> = {
        let def_path = PathBuf::from("./../").join("meta_issuer.wasm.gz");
        let err = format!("
        Could not find Meta Issuer Wasm module for current build.
        I will look for it at {:?} (note that I run from {:?}).
        You can build the Wasm by running ./build.sh in <project-home>/issuer/
        ", &def_path,
            &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_|
                "an unknown directory".to_string()));
                get_wasm_path("META_ISSUER_WASM".to_string(), &def_path).expect(&err)

    };

        /// Gzipped Wasm module for the current VC Playground RP build, i.e. the one we're testing
    pub static ref RELYING_PARTY_WASM: Vec<u8> = {
        let def_path = PathBuf::from("./../").join("relying_party.wasm.gz");
        let err = format!("
        Could not find Relying Party Wasm module for current build.
        I will look for it at {:?} (note that I run from {:?}).
        You can build the Wasm by running ./build.sh in <project-home>/rp/
        ", &def_path,
            &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_|
                "an unknown directory".to_string()));
                get_wasm_path("RELYING_PARTY_WASM".to_string(), &def_path).expect(&err)

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
        ic_root_key_der: IC_ROOT_PK_DER.to_vec(),
        idp_canister_ids: vec![Principal::from_text(DUMMY_II_CANISTER_ID).unwrap()],
        derivation_origin: DUMMY_ISSUER_DERIVATION_ORIGIN.to_string(),
        frontend_hostname: DUMMY_ISSUER_FRONTEND_HOSTNAME.to_string(),
    };

    pub static ref DUMMY_SIGNED_ID_ALIAS: SignedIssuerIdAlias = SignedIssuerIdAlias {
        credential_jws: DUMMY_ALIAS_JWS.to_string(),
    };
}

// Setup helpers.
#[derive(CandidType, Clone, Deserialize)]
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

pub fn install_canister<Init: CandidType>(
    env: &StateMachine,
    wasm: Vec<u8>,
    maybe_init: Option<Init>,
) -> CanisterId {
    let canister_id = env.create_canister(None);
    let arg = match maybe_init {
        Some(init) => candid::encode_one(Some(init)).expect("error encoding init arg as candid"),
        None => candid::encode_one("()").expect("error encoding empty arg as candid"),
    };
    env.install_canister(canister_id, wasm, arg, None);
    canister_id
}

pub fn install_issuer(env: &StateMachine, maybe_init: Option<IssuerInit>) -> CanisterId {
    install_canister(env, META_ISSUER_WASM.clone(), maybe_init)
}

pub fn do_set_user(
    user_data: UserData,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) {
    api::set_user(env, canister_id, caller, SetUserRequest { user_data })
        .expect("API call failed")
        .expect("Failed set_user");
}

pub fn do_get_user(caller: Principal, env: &StateMachine, canister_id: Principal) -> UserData {
    api::get_user(env, canister_id, caller)
        .expect("API call failed")
        .expect("Failed get_user")
}

pub fn do_group_types(caller: Principal, env: &StateMachine, canister_id: Principal) -> GroupTypes {
    api::group_types(env, canister_id, caller)
        .expect("API call failed")
        .expect("Failed group_types")
}

pub fn add_group_with_member(
    group_name: &str,
    owner: Principal,
    member: Principal,
    vc_arguments: Option<HashMap<String, ArgumentValue>>,
    env: &StateMachine,
    canister_id: Principal,
) {
    do_add_group(group_name, owner, env, canister_id);
    do_join_group(group_name, owner, member, vc_arguments, env, canister_id);
    do_update_membership(
        group_name,
        vec![MembershipUpdate {
            member,
            new_status: MembershipStatus::Accepted,
        }],
        owner,
        env,
        canister_id,
    );
}

// API helpers.
pub fn do_add_group(
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

pub fn do_get_group(
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

pub fn do_join_group(
    group_name: &str,
    owner: Principal,
    caller: Principal,
    vc_arguments: Option<HashMap<String, ArgumentValue>>,
    env: &StateMachine,
    canister_id: Principal,
) {
    api::join_group(
        env,
        canister_id,
        caller,
        JoinGroupRequest {
            group_name: group_name.to_string(),
            owner,
            vc_arguments: vc_arguments.map(|args| {
                args.into_iter()
                    .map(|(k, v)| (k, meta_issuer::groups_api::ArgumentValue::from(v)))
                    .collect()
            }),
        },
    )
    .expect("API call failed")
    .expect("Failed join_group");
}

pub fn do_update_membership(
    group_name: &str,
    updates: Vec<MembershipUpdate>,
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
            updates,
        },
    )
    .expect("API call failed")
    .expect("Failed update_membership");
}

/// Issuer API.
pub mod api {
    use super::*;
    use meta_issuer::groups_api::{GroupTypes, SetUserRequest, UserData};

    pub fn configure(
        env: &StateMachine,
        canister_id: CanisterId,
        config: &IssuerInit,
    ) -> Result<(), CallError> {
        call_candid(env, canister_id, "configure", (config,))
    }

    pub fn vc_consent_message(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        consent_message_request: &Icrc21VcConsentMessageRequest,
    ) -> Result<Result<Icrc21ConsentInfo, Icrc21Error>, CallError> {
        call_candid_as(
            env,
            canister_id,
            sender,
            "vc_consent_message",
            (consent_message_request,),
        )
        .map(|(x,)| x)
    }

    pub fn prepare_credential(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        prepare_credential_request: &PrepareCredentialRequest,
    ) -> Result<Result<PreparedCredentialData, IssueCredentialError>, CallError> {
        call_candid_as(
            env,
            canister_id,
            sender,
            "prepare_credential",
            (prepare_credential_request,),
        )
        .map(|(x,)| x)
    }

    pub fn get_credential(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        get_credential_request: &GetCredentialRequest,
    ) -> Result<Result<IssuedCredentialData, IssueCredentialError>, CallError> {
        query_candid_as(
            env,
            canister_id,
            sender,
            "get_credential",
            (get_credential_request,),
        )
        .map(|(x,)| x)
    }

    pub fn group_types(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
    ) -> Result<Result<GroupTypes, GroupsError>, CallError> {
        query_candid_as(env, canister_id, sender, "group_types", ()).map(|(x,)| x)
    }

    pub fn get_user(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
    ) -> Result<Result<UserData, GroupsError>, CallError> {
        query_candid_as(env, canister_id, sender, "get_user", ()).map(|(x,)| x)
    }

    pub fn set_user(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: SetUserRequest,
    ) -> Result<Result<(), GroupsError>, CallError> {
        call_candid_as(env, canister_id, sender, "set_user", (req,)).map(|(x,)| x)
    }

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
