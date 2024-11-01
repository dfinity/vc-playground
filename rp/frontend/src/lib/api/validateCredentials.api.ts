import { getRpCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import type { CredentialSpec } from '../../declarations/rp/rp.did';
import { nonNullish } from '$lib/utils/non-nullish';

type ValidateCredentialsRequest = {
  // The principal of the user that requests the credentials.
  vcSubject: Principal;
  // The domain of the frontend of the issuer of the credentials.
  issuerOrigin: string;
  issuerCanisterId?: Principal;
  // The JSON credentials as received by the issuer.
  vpJwt: string;
  // The same parameters as when the credentials was requested.
  credentialSpec: CredentialSpec;
};

export const validateCredentials = async ({
  identity,
  requestParams,
}: {
  identity: Identity;
  requestParams: ValidateCredentialsRequest;
}): Promise<boolean> => {
  const actor = await getRpCanister(identity);
  const response = await actor.validate_ii_vp({
    effective_vc_subject: requestParams.vcSubject,
    issuer_origin: requestParams.issuerOrigin,
    issuer_canister_id: nonNullish(requestParams.issuerCanisterId)
      ? [requestParams.issuerCanisterId]
      : [],
    vp_jwt: requestParams.vpJwt,
    credential_spec: requestParams.credentialSpec,
  });
  if ('Err' in response) {
    console.log('Error in response: ', response.Err);
    return false;
  }
  // Then 'Ok' is in response and the credentials are valid.
  return true;
};
