use candid::{CandidType, Deserialize, Principal};
use canister_tests::framework::get_wasm_path;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_test_state_machine_client::{
    call_candid, call_candid_as, query_candid_as, CallError, StateMachine,
};
use lazy_static::lazy_static;
use meta_issuer::groups_api::{
    AddGroupRequest, FullGroupData, GetGroupRequest, GroupTypes, GroupsError, JoinGroupRequest,
    ListGroupsRequest, MembershipStatus, MembershipUpdate, PublicGroupsData, SetUserRequest,
    UpdateMembershipRequest, UserData,
};
use std::collections::HashMap;
use std::path::PathBuf;
use vc_util::issuer_api::{
    ArgumentValue, GetCredentialRequest, Icrc21ConsentInfo, Icrc21Error,
    Icrc21VcConsentMessageRequest, IssueCredentialError, IssuedCredentialData,
    PrepareCredentialRequest, PreparedCredentialData, SignedIdAlias as SignedIssuerIdAlias,
};

pub const DUMMY_ROOT_KEY: &str ="308182301d060d2b0601040182dc7c0503010201060c2b0601040182dc7c05030201036100adf65638a53056b2222c91bb2457b0274bca95198a5acbdadfe7fd72178f069bdea8d99e9479d8087a2686fc81bf3c4b11fe275570d481f1698f79d468afe0e57acc1e298f8b69798da7a891bbec197093ec5f475909923d48bfed6843dbed1f";
pub const DUMMY_II_CANISTER_ID: &str = "rwlgt-iiaaa-aaaaa-aaaaa-cai";

/// Dummy alias JWS for testing, valid wrt DUMMY_ROOT_KEY and DUMMY_II_CANISTER_ID.
/// id dapp: nugva-s7c6v-4yszt-koycv-5b623-an7q6-ha2nz-kz6rs-hawgl-nznbe-rqe
/// id alias: vhbib-m4hm6-hpvyc-7prd2-siivo-nbd7r-67o5x-n3awh-qsmqz-wznjf-tqe
const DUMMY_ALIAS_JWS: &str ="eyJqd2siOnsia3R5Ijoib2N0IiwiYWxnIjoiSWNDcyIsImsiOiJNRHd3REFZS0t3WUJCQUdEdUVNQkFnTXNBQW9BQUFBQUFBQUFBQUVCMGd6TTVJeXFMYUhyMDhtQTRWd2J5SmRxQTFyRVFUX2xNQnVVbmN5UDVVYyJ9LCJraWQiOiJkaWQ6aWNwOnJ3bGd0LWlpYWFhLWFhYWFhLWFhYWFhLWNhaSIsImFsZyI6IkljQ3MifQ.eyJleHAiOjE2MjAzMjk1MzAsImlzcyI6Imh0dHBzOi8vaWRlbnRpdHkuaWMwLmFwcC8iLCJuYmYiOjE2MjAzMjg2MzAsImp0aSI6ImRhdGE6dGV4dC9wbGFpbjtjaGFyc2V0PVVURi04LHRpbWVzdGFtcF9uczoxNjIwMzI4NjMwMDAwMDAwMDAwLGFsaWFzX2hhc2g6YTI3YzU4NTQ0MmUwN2RkZWFkZTRjNWE0YTAzMjdkMzA4NTE5NDAzYzRlYTM3NDIxNzBhZTRkYzk1YjIyZTQ3MyIsInN1YiI6ImRpZDppY3A6bnVndmEtczdjNnYtNHlzenQta295Y3YtNWI2MjMtYW43cTYtaGEybnota3o2cnMtaGF3Z2wtbnpuYmUtcnFlIiwidmMiOnsiQGNvbnRleHQiOiJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJJbnRlcm5ldElkZW50aXR5SWRBbGlhcyJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJJbnRlcm5ldElkZW50aXR5SWRBbGlhcyI6eyJoYXNJZEFsaWFzIjoiamtrMjItenFkeGMta2dwZXotNnN2Mm0tNXBieTQtd2k0dDItcHJtb3EtZ2YyaWgtaTJxdGMtdjM3YWMtNWFlIn19fX0.2dn3omtjZXJ0aWZpY2F0ZVkBsdnZ96JkdHJlZYMBgwGDAYMCSGNhbmlzdGVygwGDAkoAAAAAAAAAAAEBgwGDAYMBgwJOY2VydGlmaWVkX2RhdGGCA1ggvlJBTZDgK1_9Vb3-18dWKIfy28WTjZ1YqdjFWWAIX96CBFgg0sz_P8xdqTDewOhKJUHmWFFrS7FQHnDotBDmmGoFfWCCBFgg_KZ0TVqubo_EGWoMUPA35BYZ4B5ZRkR_zDfNIQCwa46CBFggj_ZV-7o59iVEjztzZtpNnO9YC7GjbKmg2eDtJzGz1weCBFggXAzCWvb9h4qsVs41IUJBABzjSqAZ8DIzF_ghGHpGmHGCBFggJhbsbvKYt7rjLK5SI0NDc600o-ajSYQNuOXps6qUrdiCBFggBFQwZetJeY_gx6TQohTqUOskblddajS20DA0esxWoyWDAYIEWCA1U_ZYHVOz3Sdkb2HIsNoLDDiBuFfG3DxH6miIwRPra4MCRHRpbWWCA0mAuK7U3YmkvhZpc2lnbmF0dXJlWDC5cq4UxYy7cnkcw6yv5SCh4POY9u0iHecZuxO8E9oxIqXRdHmnYVF0Fv_R-aws0EBkdHJlZYMBggRYIOGnlc_3yXPTVrEJ1p3dKX5HxkMOziUnpA1HeXiQW4O8gwJDc2lngwJYIIOQR7wl3Ws9Jb8VP4rhIb37XKLMkkZ2P7WaZ5we60WGgwGCBFgg21-OewBgqt_-0AtHHHS4yPyQK9g6JTHaGUuSIw4QYgqDAlgg5bQnHHvS3FfM_BaiSL6n19qoXkuA1KoLWk963fOUMW-CA0A";
pub const DUMMY_ALIAS_ID_DAPP_PRINCIPAL: &str =
    "nugva-s7c6v-4yszt-koycv-5b623-an7q6-ha2nz-kz6rs-hawgl-nznbe-rqe";

pub const DUMMY_DERIVATION_ORIGIN: &str = "https://y2aaj-miaaa-aaaad-aacxq-cai.ic0.app";
pub const DUMMY_FRONTEND_HOSTNAME: &str = "https://y2aaj-miaaa-aaaad-aacxq-cai.ic0.app";

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
        ic_root_key_der: hex::decode(DUMMY_ROOT_KEY).unwrap(),
        idp_canister_ids: vec![Principal::from_text(DUMMY_II_CANISTER_ID).unwrap()],
        derivation_origin: DUMMY_DERIVATION_ORIGIN.to_string(),
        frontend_hostname: DUMMY_FRONTEND_HOSTNAME.to_string(),
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
