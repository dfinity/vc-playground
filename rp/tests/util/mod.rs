use candid::{CandidType, Deserialize, Principal};
use canister_tests::framework::get_wasm_path;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_test_state_machine_client::{call_candid, call_candid_as, CallError, StateMachine};
use lazy_static::lazy_static;
use relying_party::rp_api::{
    AddExclusiveContentRequest, ContentData, ContentError, ExclusiveContentList, ImagesList,
    ListExclusiveContentRequest, ListImagesRequest,
};
use std::path::PathBuf;

lazy_static! {
    /// Gzipped Wasm module for the current Early Adopter Issuer build, i.e. the one we're testing
    pub static ref RELYING_PARTY_WASM: Vec<u8> = {
        let def_path = PathBuf::from("./../").join("relying_party.wasm.gz");
        let err = format!("
        Could not find Early Adopter Issuer Wasm module for current build.
        I will look for it at {:?} (note that I run from {:?}).
        You can build the Wasm by running ./build.sh
        ", &def_path,
            &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_|
                "an unknown directory".to_string()));
                get_wasm_path("RELYING_PARTY_WASM".to_string(), &def_path).expect(&err)

    };

}

// Setup helpers.
#[derive(CandidType, Clone, Deserialize)]
pub struct RpConfig {}

pub fn install_canister(env: &StateMachine, wasm: Vec<u8>) -> CanisterId {
    let canister_id = env.create_canister(None);
    let arg = candid::encode_one("()").expect("error encoding empty arg as candid");
    env.install_canister(canister_id, wasm, arg, None);
    canister_id
}

pub fn install_rp(env: &StateMachine, maybe_init: Option<RpConfig>) -> CanisterId {
    let canister_id = env.create_canister(None);
    let arg = match maybe_init {
        Some(init) => {
            candid::encode_one(Some(init)).expect("error encoding issuer init arg as candid")
        }
        None => candid::encode_one("()").expect("error encoding empty arg as candid"),
    };
    env.install_canister(canister_id, RELYING_PARTY_WASM.clone(), arg, None);
    canister_id
}

// API helpers.
pub fn do_add_exclusive_content(
    content_name: &str,
    url: &str,
    credential_group_name: &str,
    caller: Principal,
    env: &StateMachine,
    canister_id: Principal,
) -> ContentData {
    api::add_exclusive_content(
        env,
        canister_id,
        caller,
        AddExclusiveContentRequest {
            content_name: content_name.to_string(),
            url: url.to_string(),
            credential_group_name: credential_group_name.to_string(),
        },
    )
    .expect("API call failed")
    .expect("Failed add_exclusive_content")
}

pub fn do_list_images(env: &StateMachine, canister_id: Principal) -> ImagesList {
    api::list_images(env, canister_id, &ListImagesRequest {})
        .expect("API call failed")
        .expect("Failed list_images")
}

pub fn do_list_exclusive_content(
    env: &StateMachine,
    maybe_owner: Option<Principal>,
    canister_id: Principal,
) -> ExclusiveContentList {
    api::list_exclusive_content(
        env,
        canister_id,
        &ListExclusiveContentRequest {
            owned_by: maybe_owner,
        },
    )
    .expect("API call failed")
    .expect("Failed list_exclusive_content")
}

/// Relying party API.
pub mod api {
    use super::*;
    use ic_test_state_machine_client::query_candid;
    use relying_party::rp_api::{AddExclusiveContentRequest, ContentData};

    pub fn configure(
        env: &StateMachine,
        canister_id: CanisterId,
        config: &RpConfig,
    ) -> Result<(), CallError> {
        call_candid(env, canister_id, "configure", (config,))
    }

    pub fn list_images(
        env: &StateMachine,
        canister_id: CanisterId,
        req: &ListImagesRequest,
    ) -> Result<Result<ImagesList, ContentError>, CallError> {
        query_candid(env, canister_id, "list_images", (req,)).map(|(x,)| x)
    }

    pub fn list_exclusive_content(
        env: &StateMachine,
        canister_id: CanisterId,
        req: &ListExclusiveContentRequest,
    ) -> Result<Result<ExclusiveContentList, ContentError>, CallError> {
        query_candid(env, canister_id, "list_exclusive_content", (req,)).map(|(x,)| x)
    }

    pub fn add_exclusive_content(
        env: &StateMachine,
        canister_id: CanisterId,
        sender: Principal,
        req: AddExclusiveContentRequest,
    ) -> Result<Result<ContentData, ContentError>, CallError> {
        call_candid_as(env, canister_id, sender, "add_exclusive_content", (req,)).map(|(x,)| x)
    }
}
