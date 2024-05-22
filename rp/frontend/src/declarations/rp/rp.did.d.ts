import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddExclusiveContentRequest {
  'url' : string,
  'credential_issuer' : Principal,
  'content_name' : string,
  'credential_group_name' : string,
  'credential_spec' : CredentialSpec,
}
export type ArgumentValue = { 'Int' : number } |
  { 'String' : string };
export interface ContentData {
  'url' : string,
  'credential_issuer' : Principal,
  'owner' : Principal,
  'content_name' : string,
  'credential_group_name' : string,
  'created_timestamp_ns' : TimestampNs,
  'credential_spec' : CredentialSpec,
}
export type ContentError = { 'Internal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'AlreadyExists' : string };
export interface CredentialSpec {
  'arguments' : [] | [Array<[string, ArgumentValue]>],
  'credential_type' : string,
}
export interface ExclusiveContentList { 'content_items' : Array<ContentData> }
export type HeaderField = [string, string];
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'certificate_version' : [] | [number],
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'status_code' : number,
}
export interface ImageData { 'url' : string }
export interface ImagesList { 'images' : Array<ImageData> }
export interface IssuerData { 'canister_id' : Principal, 'vc_url' : string }
export interface ListExclusiveContentRequest { 'owned_by' : [] | [Principal] }
export type ListImagesRequest = {};
export interface RpInit {
  'ii_canister_id' : Principal,
  'ic_root_key_der' : Uint8Array | number[],
  'issuers' : Array<IssuerData>,
  'ii_vc_url' : string,
}
export type TimestampNs = bigint;
export type UploadImagesRequest = {};
export interface ValidateVpRequest {
  'effective_vc_subject' : Principal,
  'issuer_origin' : string,
  'issuer_canister_id' : [] | [Principal],
  'vp_jwt' : string,
  'credential_spec' : CredentialSpec,
}
export interface _SERVICE {
  'add_exclusive_content' : ActorMethod<
    [AddExclusiveContentRequest],
    { 'Ok' : ContentData } |
      { 'Err' : ContentError }
  >,
  'configure' : ActorMethod<[RpInit], undefined>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'list_exclusive_content' : ActorMethod<
    [ListExclusiveContentRequest],
    { 'Ok' : ExclusiveContentList } |
      { 'Err' : ContentError }
  >,
  'list_images' : ActorMethod<
    [ListImagesRequest],
    { 'Ok' : ImagesList } |
      { 'Err' : ContentError }
  >,
  'upload_images' : ActorMethod<
    [UploadImagesRequest],
    { 'Ok' : ImagesList } |
      { 'Err' : ContentError }
  >,
  'validate_ii_vp' : ActorMethod<
    [ValidateVpRequest],
    { 'Ok' : null } |
      { 'Err' : ContentError }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
