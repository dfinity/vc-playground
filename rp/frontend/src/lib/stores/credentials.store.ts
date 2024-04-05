import type { Principal } from '@dfinity/principal';
import { writable } from 'svelte/store';

type Credential = {
  groupName: string;
  owner: Principal;
  timestampMillis: number;
  hasCredential: boolean;
};
type CredentialsStoreData = Record<string, Credential>;

const initStore = () => {
  const { update, subscribe, set } = writable<CredentialsStoreData>({});

  return {
    subscribe,
    reset: () => {
      set({});
    },
    setCredential: ({
      groupName,
      owner,
      hasCredential,
    }: {
      groupName: string;
      owner: Principal;
      hasCredential: boolean;
    }) =>
      update((storeData) => {
        const key = `${groupName}-${owner.toText()}`;
        return {
          ...storeData,
          [key]: {
            groupName,
            owner,
            timestampMillis: Date.now(),
            hasCredential,
          },
        };
      }),
  };
};

export const credentialsStore = initStore();
