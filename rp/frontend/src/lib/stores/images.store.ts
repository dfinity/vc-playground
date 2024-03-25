import { writable, type Writable } from 'svelte/store';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import { queryImages } from '$lib/api/queryImages.api';
import type { ImageData } from '../../declarations/rp/rp.did';

const imagesStores: Record<string, Writable<ImageData[] | undefined>> = {};
export const getImagesStore = (
  identity: Identity | undefined | null
): Writable<ImageData[] | undefined> => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!imagesStores[identityPrincipal]) {
    imagesStores[identityPrincipal] = writable<ImageData[] | undefined>(
      undefined,
      (_set, update) => {
        if (browser) {
          queryImages({ identity: identity ?? new AnonymousIdentity() }).then(({ images }) => {
            update(() => images);
          });
        }
      }
    );
  }
  return imagesStores[identityPrincipal];
};
