import type { Credential } from '$lib/stores/credentials.store';
import type { Principal } from '@dfinity/principal';
import { equalCredentials } from './equal-credentials.utils';
import type { CredentialSpec } from '../../declarations/rp/rp.did';

export const findCredential =
  ({ credentialSpec, owner }: { credentialSpec?: CredentialSpec; owner?: Principal }) =>
  (credentialStored: Credential) => {
    if (!credentialSpec || !owner) {
      return false;
    }
    if (owner.compareTo(credentialStored.owner) !== 'eq') {
      return false;
    }
    return equalCredentials(credentialSpec, credentialStored.credentialSpec);
  };
