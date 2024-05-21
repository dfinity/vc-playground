export const idlFactory = ({ IDL }) => {
  const IssuerInit = IDL.Record({
    'derivation_origin' : IDL.Text,
    'idp_canister_ids' : IDL.Vec(IDL.Principal),
    'ic_root_key_der' : IDL.Vec(IDL.Nat8),
    'frontend_hostname' : IDL.Text,
  });
  const AddGroupRequest = IDL.Record({ 'group_name' : IDL.Text });
  const MembershipStatus = IDL.Variant({
    'PendingReview' : IDL.Null,
    'Rejected' : IDL.Null,
    'Accepted' : IDL.Null,
  });
  const TimestampNs = IDL.Nat64;
  const ArgumentValue = IDL.Variant({ 'Int' : IDL.Int32, 'String' : IDL.Text });
  const VcArguments = IDL.Vec(IDL.Tuple(IDL.Text, ArgumentValue));
  const MemberData = IDL.Record({
    'member' : IDL.Principal,
    'membership_status' : MembershipStatus,
    'nickname' : IDL.Text,
    'joined_timestamp_ns' : TimestampNs,
    'vc_arguments' : IDL.Opt(VcArguments),
  });
  const GroupStats = IDL.Record({
    'created_timestamp_ns' : TimestampNs,
    'member_count' : IDL.Nat32,
  });
  const FullGroupData = IDL.Record({
    'members' : IDL.Vec(MemberData),
    'owner' : IDL.Principal,
    'stats' : GroupStats,
    'issuer_nickname' : IDL.Text,
    'group_name' : IDL.Text,
  });
  const GroupsError = IDL.Variant({
    'Internal' : IDL.Text,
    'NotFound' : IDL.Text,
    'NotAuthorized' : IDL.Text,
    'AlreadyExists' : IDL.Text,
    'NotAuthenticated' : IDL.Text,
  });
  const DerivationOriginRequest = IDL.Record({
    'frontend_hostname' : IDL.Text,
  });
  const DerivationOriginData = IDL.Record({ 'origin' : IDL.Text });
  const DerivationOriginError = IDL.Variant({
    'Internal' : IDL.Text,
    'UnsupportedOrigin' : IDL.Text,
  });
  const SignedIdAlias = IDL.Record({ 'credential_jws' : IDL.Text });
  const CredentialSpec = IDL.Record({
    'arguments' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, ArgumentValue))),
    'credential_type' : IDL.Text,
  });
  const GetCredentialRequest = IDL.Record({
    'signed_id_alias' : SignedIdAlias,
    'prepared_context' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'credential_spec' : CredentialSpec,
  });
  const IssuedCredentialData = IDL.Record({ 'vc_jws' : IDL.Text });
  const IssueCredentialError = IDL.Variant({
    'Internal' : IDL.Text,
    'SignatureNotFound' : IDL.Text,
    'InvalidIdAlias' : IDL.Text,
    'UnauthorizedSubject' : IDL.Text,
    'UnknownSubject' : IDL.Text,
    'UnsupportedCredentialSpec' : IDL.Text,
  });
  const GetGroupRequest = IDL.Record({ 'group_name' : IDL.Text });
  const UserData = IDL.Record({
    'user_nickname' : IDL.Opt(IDL.Text),
    'issuer_nickname' : IDL.Opt(IDL.Text),
  });
  const GroupType = IDL.Record({
    'group_name' : IDL.Text,
    'credential_spec' : CredentialSpec,
  });
  const GroupTypes = IDL.Record({ 'types' : IDL.Vec(GroupType) });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'certificate_version' : IDL.Opt(IDL.Nat16),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'status_code' : IDL.Nat16,
  });
  const JoinGroupRequest = IDL.Record({
    'owner' : IDL.Principal,
    'vc_arguments' : IDL.Opt(VcArguments),
    'group_name' : IDL.Text,
  });
  const ListGroupsRequest = IDL.Record({
    'group_name_substring' : IDL.Opt(IDL.Text),
  });
  const PublicGroupData = IDL.Record({
    'membership_status' : IDL.Opt(MembershipStatus),
    'owner' : IDL.Principal,
    'vc_arguments' : IDL.Opt(VcArguments),
    'stats' : GroupStats,
    'issuer_nickname' : IDL.Text,
    'group_name' : IDL.Text,
  });
  const PublicGroupsData = IDL.Record({ 'groups' : IDL.Vec(PublicGroupData) });
  const PrepareCredentialRequest = IDL.Record({
    'signed_id_alias' : SignedIdAlias,
    'credential_spec' : CredentialSpec,
  });
  const PreparedCredentialData = IDL.Record({
    'prepared_context' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const SetUserRequest = IDL.Record({ 'user_data' : UserData });
  const MembershipUpdate = IDL.Record({
    'member' : IDL.Principal,
    'new_status' : MembershipStatus,
  });
  const UpdateMembershipRequest = IDL.Record({
    'updates' : IDL.Vec(MembershipUpdate),
    'group_name' : IDL.Text,
  });
  const Icrc21ConsentPreferences = IDL.Record({ 'language' : IDL.Text });
  const Icrc21VcConsentMessageRequest = IDL.Record({
    'preferences' : Icrc21ConsentPreferences,
    'credential_spec' : CredentialSpec,
  });
  const Icrc21ConsentInfo = IDL.Record({
    'consent_message' : IDL.Text,
    'language' : IDL.Text,
  });
  const Icrc21ErrorInfo = IDL.Record({ 'description' : IDL.Text });
  const Icrc21Error = IDL.Variant({
    'GenericError' : IDL.Record({
      'description' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'UnsupportedCanisterCall' : Icrc21ErrorInfo,
    'ConsentMessageUnavailable' : Icrc21ErrorInfo,
  });
  return IDL.Service({
    'add_group' : IDL.Func(
        [AddGroupRequest],
        [IDL.Variant({ 'Ok' : FullGroupData, 'Err' : GroupsError })],
        [],
      ),
    'configure' : IDL.Func([IssuerInit], [], []),
    'derivation_origin' : IDL.Func(
        [DerivationOriginRequest],
        [
          IDL.Variant({
            'Ok' : DerivationOriginData,
            'Err' : DerivationOriginError,
          }),
        ],
        [],
      ),
    'get_credential' : IDL.Func(
        [GetCredentialRequest],
        [
          IDL.Variant({
            'Ok' : IssuedCredentialData,
            'Err' : IssueCredentialError,
          }),
        ],
        ['query'],
      ),
    'get_group' : IDL.Func(
        [GetGroupRequest],
        [IDL.Variant({ 'Ok' : FullGroupData, 'Err' : GroupsError })],
        ['query'],
      ),
    'get_user' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : UserData, 'Err' : GroupsError })],
        ['query'],
      ),
    'group_types' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : GroupTypes, 'Err' : GroupsError })],
        ['query'],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'join_group' : IDL.Func(
        [JoinGroupRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : GroupsError })],
        [],
      ),
    'list_groups' : IDL.Func(
        [ListGroupsRequest],
        [IDL.Variant({ 'Ok' : PublicGroupsData, 'Err' : GroupsError })],
        ['query'],
      ),
    'prepare_credential' : IDL.Func(
        [PrepareCredentialRequest],
        [
          IDL.Variant({
            'Ok' : PreparedCredentialData,
            'Err' : IssueCredentialError,
          }),
        ],
        [],
      ),
    'set_user' : IDL.Func(
        [SetUserRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : GroupsError })],
        [],
      ),
    'update_membership' : IDL.Func(
        [UpdateMembershipRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : GroupsError })],
        [],
      ),
    'vc_consent_message' : IDL.Func(
        [Icrc21VcConsentMessageRequest],
        [IDL.Variant({ 'Ok' : Icrc21ConsentInfo, 'Err' : Icrc21Error })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const IssuerInit = IDL.Record({
    'derivation_origin' : IDL.Text,
    'idp_canister_ids' : IDL.Vec(IDL.Principal),
    'ic_root_key_der' : IDL.Vec(IDL.Nat8),
    'frontend_hostname' : IDL.Text,
  });
  return [IDL.Opt(IssuerInit)];
};
