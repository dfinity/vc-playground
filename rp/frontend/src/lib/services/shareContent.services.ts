import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { addExclusiveContent } from '$lib/api/addExclusiveContent.api';
import type { ImageData } from '../../declarations/rp/rp.did';
import type { Principal } from '@dfinity/principal';

export const shareContent = async ({
  identity,
  image,
  issuerName,
  owner,
}: {
  identity: Identity | null | undefined;
  image: ImageData;
  issuerName: string;
  owner: Principal;
}) => {
  try {
    if (isNullish(identity)) {
      throw new Error('No identity found');
    }
    await addExclusiveContent({
      identity,
      issuerName,
      owner,
      url: image.url,
      contentName: Date.now().toString(),
    });
  } catch (err: unknown) {
    // TODO: Handle error
    console.error(err);
  }
};
