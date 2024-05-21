import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddGroupRequest { 'group_name' : string }
export type ArgumentValue = { 'Int' : number } |
  { 'String' : string };
export interface CredentialSpec {
  'arguments' : [] | [Array<[string, ArgumentValue]>],
  'credential_type' : string,
}
export interface DerivationOriginData { 'origin' : string }
export type DerivationOriginError = { 'Internal' : string } |
  { 'UnsupportedOrigin' : string };
export interface DerivationOriginRequest { 'frontend_hostname' : string }
export interface FullGroupData {
  'members' : Array<MemberData>,
  'owner' : Principal,
  'stats' : GroupStats,
  'issuer_nickname' : string,
  'group_name' : string,
}
export interface GetCredentialRequest {
  'signed_id_alias' : SignedIdAlias,
  'prepared_context' : [] | [Uint8Array | number[]],
  'credential_spec' : CredentialSpec,
}
export interface GetGroupRequest { 'group_name' : string }
export interface GroupStats {
  'created_timestamp_ns' : TimestampNs,
  'member_count' : number,
}
export interface GroupType {
  'group_name' : string,
  'credential_spec' : CredentialSpec,
}
export interface GroupTypes { 'types' : Array<GroupType> }
export type GroupsError = { 'Internal' : string } |
  { 'NotFound' : string } |
  { 'NotAuthorized' : string } |
  { 'AlreadyExists' : string } |
  { 'NotAuthenticated' : string };
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
export interface Icrc21ConsentInfo {
  'consent_message' : string,
  'language' : string,
}
export interface Icrc21ConsentPreferences { 'language' : string }
export type Icrc21Error = {
    'GenericError' : { 'description' : string, 'error_code' : bigint }
  } |
  { 'UnsupportedCanisterCall' : Icrc21ErrorInfo } |
  { 'ConsentMessageUnavailable' : Icrc21ErrorInfo };
export interface Icrc21ErrorInfo { 'description' : string }
export interface Icrc21VcConsentMessageRequest {
  'preferences' : Icrc21ConsentPreferences,
  'credential_spec' : CredentialSpec,
}
export type IssueCredentialError = { 'Internal' : string } |
  { 'SignatureNotFound' : string } |
  { 'InvalidIdAlias' : string } |
  { 'UnauthorizedSubject' : string } |
  { 'UnknownSubject' : string } |
  { 'UnsupportedCredentialSpec' : string };
export interface IssuedCredentialData { 'vc_jws' : string }
export interface IssuerInit {
  'derivation_origin' : string,
  'idp_canister_ids' : Array<Principal>,
  'ic_root_key_der' : Uint8Array | number[],
  'frontend_hostname' : string,
}
export interface JoinGroupRequest {
  'owner' : Principal,
  'vc_arguments' : [] | [VcArguments],
  'group_name' : string,
}
export interface ListGroupsRequest { 'group_name_substring' : [] | [string] }
export interface MemberData {
  'member' : Principal,
  'membership_status' : MembershipStatus,
  'nickname' : string,
  'joined_timestamp_ns' : TimestampNs,
  'vc_arguments' : [] | [VcArguments],
}
export type MembershipStatus = { 'PendingReview' : null } |
  { 'Rejected' : null } |
  { 'Accepted' : null };
export interface MembershipUpdate {
  'member' : Principal,
  'new_status' : MembershipStatus,
}
export interface PrepareCredentialRequest {
  'signed_id_alias' : SignedIdAlias,
  'credential_spec' : CredentialSpec,
}
export interface PreparedCredentialData {
  'prepared_context' : [] | [Uint8Array | number[]],
}
export interface PublicGroupData {
  'membership_status' : [] | [MembershipStatus],
  'owner' : Principal,
  'vc_arguments' : [] | [VcArguments],
  'stats' : GroupStats,
  'issuer_nickname' : string,
  'group_name' : string,
}
export interface PublicGroupsData { 'groups' : Array<PublicGroupData> }
export interface SetUserRequest { 'user_data' : UserData }
export interface SignedIdAlias { 'credential_jws' : string }
export type TimestampNs = bigint;
export interface UpdateMembershipRequest {
  'updates' : Array<MembershipUpdate>,
  'group_name' : string,
}
export interface UserData {
  'user_nickname' : [] | [string],
  'issuer_nickname' : [] | [string],
}
export type VcArguments = Array<[string, ArgumentValue]>;
export interface _SERVICE {
  'add_group' : ActorMethod<
    [AddGroupRequest],
    { 'Ok' : FullGroupData } |
      { 'Err' : GroupsError }
  >,
  'configure' : ActorMethod<[IssuerInit], undefined>,
  'derivation_origin' : ActorMethod<
    [DerivationOriginRequest],
    { 'Ok' : DerivationOriginData } |
      { 'Err' : DerivationOriginError }
  >,
  'get_credential' : ActorMethod<
    [GetCredentialRequest],
    { 'Ok' : IssuedCredentialData } |
      { 'Err' : IssueCredentialError }
  >,
  'get_group' : ActorMethod<
    [GetGroupRequest],
    { 'Ok' : FullGroupData } |
      { 'Err' : GroupsError }
  >,
  'get_user' : ActorMethod<[], { 'Ok' : UserData } | { 'Err' : GroupsError }>,
  'group_types' : ActorMethod<
    [],
    { 'Ok' : GroupTypes } |
      { 'Err' : GroupsError }
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'join_group' : ActorMethod<
    [JoinGroupRequest],
    { 'Ok' : null } |
      { 'Err' : GroupsError }
  >,
  'list_groups' : ActorMethod<
    [ListGroupsRequest],
    { 'Ok' : PublicGroupsData } |
      { 'Err' : GroupsError }
  >,
  'prepare_credential' : ActorMethod<
    [PrepareCredentialRequest],
    { 'Ok' : PreparedCredentialData } |
      { 'Err' : IssueCredentialError }
  >,
  'set_user' : ActorMethod<
    [SetUserRequest],
    { 'Ok' : null } |
      { 'Err' : GroupsError }
  >,
  'update_membership' : ActorMethod<
    [UpdateMembershipRequest],
    { 'Ok' : null } |
      { 'Err' : GroupsError }
  >,
  'vc_consent_message' : ActorMethod<
    [Icrc21VcConsentMessageRequest],
    { 'Ok' : Icrc21ConsentInfo } |
      { 'Err' : Icrc21Error }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
