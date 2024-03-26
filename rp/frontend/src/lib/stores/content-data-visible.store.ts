import { derived, type Readable } from 'svelte/store';
import type { ContentData } from '../../declarations/rp/rp.did';
import { getExclusiveContentDataSortedByCreatedTimestamp } from './content-data.store';
import type { Identity } from '@dfinity/agent';
import { credentialsStore } from './credentials.store';

export type VisibleContentData = ContentData & {
  visible: boolean;
};

export const getVisibleContentData = (
  identity: Identity | null | undefined
): Readable<VisibleContentData[]> =>
  derived(
    [getExclusiveContentDataSortedByCreatedTimestamp(identity), credentialsStore],
    ([$contentData, credentials]) => {
      if (!$contentData) return [];
      return $contentData.map((contentData) => ({
        ...contentData,
        visible: credentials[contentData.credential_group_name]?.hasCredential ?? false,
      }));
    }
  );
