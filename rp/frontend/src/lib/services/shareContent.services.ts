import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { addExclusiveContent } from '$lib/api/addExclusiveContent.api';
import type { ImageData } from '../../declarations/rp/rp.did';
import type { Principal } from '@dfinity/principal';
import type { ToastStore } from '@skeletonlabs/skeleton';

export const shareContent = async ({
  identity,
  image,
  issuerName,
  owner,
  toastStore,
}: {
  identity: Identity | null | undefined;
  image: ImageData;
  issuerName: string;
  owner: Principal;
  toastStore: ToastStore;
}) => {
  try {
    await addExclusiveContent({
      identity: identity ?? new AnonymousIdentity(),
      issuerName,
      owner,
      url: image.url,
      contentName: Date.now().toString(),
    });
  } catch (err: unknown) {
    console.error('Error sharing content', err);
    toastStore.trigger({
      message: `Oops! There was an error sharing the content. Please try again. ${err}`,
      background: 'variant-filled-error',
    });
  }
};
