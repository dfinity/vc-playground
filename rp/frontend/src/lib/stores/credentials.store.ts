import type { Principal } from '@dfinity/principal';
import { writable } from 'svelte/store';
import type { CredentialSpec } from '../../declarations/rp/rp.did';

export type Credential = {
  groupName: string;
  owner: Principal;
  credentialSpec: CredentialSpec;
  timestampMillis: number;
  hasCredential: boolean;
};
type CredentialsStoreData = Credential[];

const initStore = () => {
  const { update, subscribe, set } = writable<CredentialsStoreData>([]);

  return {
    subscribe,
    reset: () => {
      set([]);
    },
    setCredential: ({
      groupName,
      owner,
      credentialSpec,
      hasCredential,
    }: {
      groupName: string;
      owner: Principal;
      credentialSpec: CredentialSpec;
      hasCredential: boolean;
    }) =>
      update((storeData) => {
        return storeData.concat([
          {
            groupName,
            credentialSpec,
            owner,
            timestampMillis: Date.now(),
            hasCredential,
          },
        ]);
      }),
  };
};

export const credentialsStore = initStore();
