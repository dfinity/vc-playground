import type { Identity } from '@dfinity/agent';
import { writable } from 'svelte/store';

type AuthStoreData = {
	// `undefiened` means we don't know if the user is authenticated or not.
	// `null` means the user is not authenticated.
	// `Identity` means the user is authenticated.
	identity: undefined | Identity | null;
};
const initData = { identity: undefined };
export const authStore = writable<AuthStoreData>(initData);
