export const idlFactory = ({ IDL }) => {
  const RpConfig = IDL.Record({});
  const AddExclusiveContentRequest = IDL.Record({
    'url' : IDL.Text,
    'content_name' : IDL.Text,
    'credential_group_name' : IDL.Text,
  });
  const TimestampNs = IDL.Nat64;
  const ContentData = IDL.Record({
    'url' : IDL.Text,
    'owner' : IDL.Principal,
    'content_name' : IDL.Text,
    'credential_group_name' : IDL.Text,
    'created_timestamp_ns' : TimestampNs,
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
  return IDL.Service({
    'add_exclusive_content' : IDL.Func(
        [AddExclusiveContentRequest],
        [IDL.Variant({ 'Ok' : ContentData, 'Err' : ContentError })],
        [],
      ),
    'configure' : IDL.Func([RpConfig], [], []),
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
  });
};
export const init = ({ IDL }) => {
  const RpConfig = IDL.Record({});
  return [IDL.Opt(RpConfig)];
};
