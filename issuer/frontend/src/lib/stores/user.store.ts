import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import type { UserData } from '../../declarations/meta_issuer.did';
import { derived, writable, type Readable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';
import { getOrCreateUser } from '$lib/services/get-or-create-user.services';

const usersStore: Record<string, Writable<UserData | undefined>> = {};
export const getUserStore = (
  identity: Identity | undefined | null
): Writable<UserData | undefined> => {
  const identityPrincipal = identity?.getPrincipal().toText();
  if (!identityPrincipal) {
    return writable<UserData>(undefined);
  }
  if (!usersStore[identityPrincipal]) {
    usersStore[identityPrincipal] = writable<UserData>(undefined, (_set, update) => {
      if (browser) {
        getOrCreateUser({ identity: identity ?? new AnonymousIdentity() }).then((user) => {
          update(() => user);
        });
      }
    });
  }
  return usersStore[identityPrincipal];
};

/**
 * Returns `user_nickname` from the user store.
 * Returns `undefined` if the user is not loaded yet.
 * Returns `null` if the user is loaded but the `user_nickname` is not set.
 */
export const getUserNickname = (
  identity: Identity | undefined | null
): Readable<string | undefined | null> =>
  derived(getUserStore(identity), (user) => {
    if (user === undefined) {
      return undefined;
    }
    return user.user_nickname[0] ?? null;
  });

/**
 * Returns `issuer_nickname` from the user store.
 * Returns `undefined` if the user is not loaded yet.
 * Returns `null` if the user is loaded but the `user_nickname` is not set.
 */
export const getIssuerNickname = (
  identity: Identity | undefined | null
): Readable<string | undefined | null> =>
  derived(getUserStore(identity), (user) => {
    if (user === undefined) {
      return undefined;
    }
    return user.issuer_nickname[0] ?? null;
  });
