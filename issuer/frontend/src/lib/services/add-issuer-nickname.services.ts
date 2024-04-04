import { getUser } from '$lib/api/getUser.api';
import { setUser } from '$lib/api/setUser.api';
import { getUserStore } from '$lib/stores/user.store';
import { validateText } from '$lib/utils/validate-text.utils';
import type { Identity } from '@dfinity/agent';
import { get } from 'svelte/store';

export const addIssuerNickname = async ({
  identity,
  nickname,
}: {
  identity: Identity;
  nickname: string;
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
    console.log('Error adding user nickname');
    console.error(err);
  }
};
