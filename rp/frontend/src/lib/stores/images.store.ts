import { writable, type Writable } from 'svelte/store';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import { queryImages } from '$lib/api/queryImages.api';
import type { ImageData } from '../../declarations/rp/rp.did';

let imagesStore: Writable<ImageData[] | undefined> | undefined = undefined;
export const getImagesStore = (
  identity: Identity | undefined | null
): Writable<ImageData[] | undefined> => {
  if (!imagesStore) {
    imagesStore = writable<ImageData[] | undefined>(undefined, (_set, update) => {
      if (browser) {
        queryImages({ identity: identity ?? new AnonymousIdentity() }).then(({ images }) => {
          update(() => images);
        });
      }
    });
  }
  return imagesStore;
};
