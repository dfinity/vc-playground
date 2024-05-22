export const idlFactory = ({ IDL }) => {
  const IssuerData = IDL.Record({
    'canister_id' : IDL.Principal,
    'vc_url' : IDL.Text,
  });
  const RpInit = IDL.Record({
    'ii_canister_id' : IDL.Principal,
    'ic_root_key_der' : IDL.Vec(IDL.Nat8),
    'issuers' : IDL.Vec(IssuerData),
    'ii_vc_url' : IDL.Text,
  });
  const ArgumentValue = IDL.Variant({ 'Int' : IDL.Int32, 'String' : IDL.Text });
  const CredentialSpec = IDL.Record({
    'arguments' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, ArgumentValue))),
    'credential_type' : IDL.Text,
  });
  const AddExclusiveContentRequest = IDL.Record({
    'url' : IDL.Text,
    'credential_issuer' : IDL.Principal,
    'content_name' : IDL.Text,
    'credential_group_name' : IDL.Text,
    'credential_spec' : CredentialSpec,
  });
  const TimestampNs = IDL.Nat64;
  const ContentData = IDL.Record({
    'url' : IDL.Text,
    'credential_issuer' : IDL.Principal,
    'owner' : IDL.Principal,
    'content_name' : IDL.Text,
    'credential_group_name' : IDL.Text,
    'created_timestamp_ns' : TimestampNs,
    'credential_spec' : CredentialSpec,
  });
  const ContentError = IDL.Variant({
    'Internal' : IDL.Text,
    'NotFound' : IDL.Text,
    'NotAuthorized' : IDL.Text,
    'AlreadyExists' : IDL.Text,
  });
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
  const ListExclusiveContentRequest = IDL.Record({
    'owned_by' : IDL.Opt(IDL.Principal),
  });
  const ExclusiveContentList = IDL.Record({
    'content_items' : IDL.Vec(ContentData),
  });
  const ListImagesRequest = IDL.Record({});
  const ImageData = IDL.Record({ 'url' : IDL.Text });
  const ImagesList = IDL.Record({ 'images' : IDL.Vec(ImageData) });
  const UploadImagesRequest = IDL.Record({});
  const ValidateVpRequest = IDL.Record({
    'effective_vc_subject' : IDL.Principal,
    'issuer_origin' : IDL.Text,
    'issuer_canister_id' : IDL.Opt(IDL.Principal),
    'vp_jwt' : IDL.Text,
    'credential_spec' : CredentialSpec,
  });
  return IDL.Service({
    'add_exclusive_content' : IDL.Func(
        [AddExclusiveContentRequest],
        [IDL.Variant({ 'Ok' : ContentData, 'Err' : ContentError })],
        [],
      ),
    'configure' : IDL.Func([RpInit], [], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'list_exclusive_content' : IDL.Func(
        [ListExclusiveContentRequest],
        [IDL.Variant({ 'Ok' : ExclusiveContentList, 'Err' : ContentError })],
        ['query'],
      ),
    'list_images' : IDL.Func(
        [ListImagesRequest],
        [IDL.Variant({ 'Ok' : ImagesList, 'Err' : ContentError })],
        ['query'],
      ),
    'upload_images' : IDL.Func(
        [UploadImagesRequest],
        [IDL.Variant({ 'Ok' : ImagesList, 'Err' : ContentError })],
        [],
      ),
    'validate_ii_vp' : IDL.Func(
        [ValidateVpRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ContentError })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const IssuerData = IDL.Record({
    'canister_id' : IDL.Principal,
    'vc_url' : IDL.Text,
  });
  const RpInit = IDL.Record({
    'ii_canister_id' : IDL.Principal,
    'ic_root_key_der' : IDL.Vec(IDL.Nat8),
    'issuers' : IDL.Vec(IssuerData),
    'ii_vc_url' : IDL.Text,
  });
  return [IDL.Opt(RpInit)];
};
