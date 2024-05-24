import type { Credential } from '$lib/stores/credentials.store';
import type { Principal } from '@dfinity/principal';
import { equalCredentials } from './equal-credentials.utils';
import type { CredentialSpec } from '../../declarations/rp/rp.did';

export const findCredential = ({
  credentialSpec,
  owner,
  credentials,
}: {
  credentialSpec?: CredentialSpec;
  owner?: Principal;
  credentials: Credential[];
}): Credential | undefined =>
  credentials.find((credential) => {
    if (!credentialSpec || !owner) {
      return false;
    }
    if (owner.compareTo(credential.owner) !== 'eq') {
      return false;
    }
    return equalCredentials(credentialSpec, credential.credentialSpec);
  });
