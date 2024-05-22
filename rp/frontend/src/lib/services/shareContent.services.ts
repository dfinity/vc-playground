import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { addExclusiveContent } from '$lib/api/addExclusiveContent.api';
import type { CredentialSpec, ImageData } from '../../declarations/rp/rp.did';
import type { Principal } from '@dfinity/principal';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { validateText } from '$lib/utils/validate-text.utils';
import { createContentCredentialSpec } from '$lib/utils/create-content-credential-spec.utils';

export const shareContent = async ({
  identity,
  image,
  owner,
  issuerName,
  toastStore,
  predicate,
  credentialSpec,
}: {
  identity: Identity | null | undefined;
  image: ImageData;
  owner: Principal;
  issuerName: string;
  toastStore: ToastStore;
  predicate?: string | number;
  credentialSpec: CredentialSpec;
}): Promise<boolean> => {
  try {
    if (predicate && typeof predicate === 'string') {
      validateText(predicate);
    }
    const contentCredentialSpec = createContentCredentialSpec({
      predicate,
      credentialSpec,
    });
    if (!contentCredentialSpec) {
      throw new Error(`Invalid credential spec: ${credentialSpec}`);
    }
    await addExclusiveContent({
      identity: identity ?? new AnonymousIdentity(),
      credentialSpec: contentCredentialSpec,
      owner,
      issuerName,
      url: image.url,
      contentName: Date.now().toString(),
    });
    return true;
  } catch (err: unknown) {
    console.error('Error sharing content', err);
    toastStore.trigger({
      message: `Oops! There was an error sharing the content. Please try again. ${err}`,
      background: 'variant-filled-error',
    });
    return false;
  }
};
