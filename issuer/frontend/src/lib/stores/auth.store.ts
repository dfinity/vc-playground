import type { Identity } from '@dfinity/agent';
import { writable } from 'svelte/store';

type AuthStoreData = {
	identity: undefined | Identity;
};
const initData = { identity: undefined };
export const authStore = writable<AuthStoreData>(initData);
