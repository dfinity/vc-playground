use candid::{CandidType, Deserialize, Principal};
use serde_bytes::ByteBuf;

/// Types for requesting a list of available images.
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ListImagesRequest {}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct UploadImagesRequest {}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ImageData {
    pub url: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ImagesList {
    pub images: Vec<ImageData>,
}

/// Types for requesting or adding exclusive content items.
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ListExclusiveContentRequest {
    pub owned_by: Option<Principal>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ContentData {
    pub owner: Principal,
    pub content_name: String,
    pub created_timestamp_ns: u64,
    pub url: String,
    pub credential_group_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ExclusiveContentList {
    pub content_items: Vec<ContentData>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct AddExclusiveContentRequest {
    pub content_name: String,
    pub url: String,
    pub credential_group_name: String,
}

// Types related to HTTP-endpoint.
pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum ContentError {
    NotAuthorized(String),
    AlreadyExists(String),
    NotFound(String),
    Internal(String),
}
