import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddExclusiveContentRequest {
  'url' : string,
  'credential_group_name' : string,
}
export interface ContentData {
  'url' : string,
  'owner' : Principal,
  'credential_group_name' : string,
  'created_timestamp_ns' : TimestampNs,
}
export type ContentError = { 'Internal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'AlreadyExists' : string };
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
export interface ListExclusiveContentRequest { 'owned_by' : [] | [Principal] }
export type ListImagesRequest = {};
export type RpConfig = {};
export type TimestampNs = bigint;
export type UploadImagesRequest = {};
export interface _SERVICE {
  'add_exclusive_content' : ActorMethod<
    [AddExclusiveContentRequest],
    { 'Ok' : ContentData } |
      { 'Err' : ContentError }
  >,
  'configure' : ActorMethod<[RpConfig], undefined>,
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
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
