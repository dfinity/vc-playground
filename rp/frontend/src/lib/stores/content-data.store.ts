import { derived, writable, type Readable, type Writable } from 'svelte/store';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import type { ContentData } from '../../declarations/rp/rp.did';
import { queryExclusiveContent } from '$lib/api/queryExclusiveContent.api';

let contentDataStore: Writable<ContentData[] | undefined> | undefined = undefined;
export const getExclusiveContentData = (
  identity: Identity | undefined | null
): Writable<ContentData[] | undefined> => {
  if (!contentDataStore) {
    contentDataStore = writable<ContentData[] | undefined>(undefined, (_set, update) => {
      if (browser) {
        queryExclusiveContent({ identity: identity ?? new AnonymousIdentity() }).then(
          ({ content_items }) => {
            update(() => content_items);
          }
        );
      }
    });
  }
  return contentDataStore;
};

export const getExclusiveContentDataSortedByCreatedTimestamp = (
  identity: Identity | null | undefined
): Readable<ContentData[]> =>
  derived(getExclusiveContentData(identity), ($contentData) => {
    if (!$contentData) return [];
    return $contentData.sort((a, b) => Number(b.created_timestamp_ns - a.created_timestamp_ns));
  });
