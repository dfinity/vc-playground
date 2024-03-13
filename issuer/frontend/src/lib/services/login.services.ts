import { authStore } from '$lib/stores/auth.store';
import { popupCenter } from '$lib/utils/login-popup.utils';
import { AuthClient } from '@dfinity/auth-client';

export const login = async () => {
	// This service never fails. It will manage the error handling internally.
	try {
		const authClient = await AuthClient.create();
		// TODO: Set loading state
		return new Promise<void>((resolve) => {
			authClient.login({
				onSuccess: async () => {
					const identity = authClient.getIdentity();
					authStore.set({ identity });
					resolve();
				},
				onError: () => {
					// TODO: Handle error
					resolve();
				},
				identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_URL,
				windowOpenerFeatures: popupCenter(),
			});
		});
	} catch (err) {
		// TODO: Handle error
	}
};
