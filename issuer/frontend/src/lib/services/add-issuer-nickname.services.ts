import { getUser } from '$lib/api/getUser.api';
import { setUser } from '$lib/api/setUser.api';
import { getUserStore } from '$lib/stores/user.store';
import { errorToString } from '$lib/utils/error-to-string.utils';
import { validateText } from '$lib/utils/validate-text.utils';
import type { Identity } from '@dfinity/agent';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { get } from 'svelte/store';

export const addIssuerNickname = async ({
  identity,
  nickname,
  toastStore,
}: {
  identity: Identity;
  nickname: string;
  toastStore: ToastStore;
}) => {
  try {
    validateText(nickname);
    const userStore = getUserStore(identity);
    const userData = get(userStore);
    await setUser({
      identity,
      issuerNickname: nickname,
      userNickname: userData?.user_nickname[0],
    });
    const user = await getUser({ identity });
    userStore.set(user);
  } catch (err) {
    console.error(err);
    toastStore.trigger({
      message: errorToString(err),
      background: 'variant-filled-error',
    });
  }
};
