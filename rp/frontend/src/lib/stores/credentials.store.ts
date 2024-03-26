import { writable } from 'svelte/store';

type Credential = {
  groupName: string;
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
    setCredential: ({ groupName, hasCredential }: { groupName: string; hasCredential: boolean }) =>
      update((storeData) => {
        return {
          ...storeData,
          [groupName]: {
            groupName,
            timestampMillis: Date.now(),
            hasCredential,
          },
        };
      }),
  };
};

export const credentialsStore = initStore();
