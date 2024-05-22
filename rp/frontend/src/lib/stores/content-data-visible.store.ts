import { derived, type Readable } from 'svelte/store';
import type { ContentData } from '../../declarations/rp/rp.did';
import { getExclusiveContentDataSortedByCreatedTimestamp } from './content-data.store';
import type { Identity } from '@dfinity/agent';
import { credentialsStore } from './credentials.store';
import { groupsStore } from './issuers.store';
import { equalCredentials } from '$lib/utils/equal-credentials.utils';
import { getIssuerGroupNameByCredTypeStore } from './issuer-types.store';

export type VisibleContentData = ContentData & {
  visible: boolean;
  issuer_nickname?: string;
};

export const getVisibleContentData = (
  identity: Identity | null | undefined
): Readable<VisibleContentData[]> =>
  derived(
    [
      getExclusiveContentDataSortedByCreatedTimestamp(identity),
      credentialsStore,
      groupsStore,
      getIssuerGroupNameByCredTypeStore(identity),
    ],
    ([$contentData, credentials, groups, groupTypesMap]) => {
      if (!$contentData) return [];
      return $contentData.map((contentData) => ({
        ...contentData,
        visible:
          credentials.find(
            (credential) =>
              equalCredentials(credential.credentialSpec, contentData.credential_spec) &&
              credential.owner.compareTo(contentData.credential_issuer) === 'eq'
          )?.hasCredential ?? false,
        issuer_nickname: groups?.find((group) => {
          return (
            group.group_name === groupTypesMap[contentData.credential_spec.credential_type] &&
            group.owner.compareTo(contentData.credential_issuer) === 'eq'
          );
        })?.issuer_nickname,
      }));
    }
  );
