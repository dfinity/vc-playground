import { queryGroup } from '$lib/api/queryGroup.api';
import { getIssuerDetailStore } from '$lib/stores/issuer-detail.store';
import { type Identity } from '@dfinity/agent';

export const loadIssuerDetail = async ({
  identity,
  issuerName,
}: {
  identity: Identity;
  issuerName: string;
}) => {
  try {
    const group = await queryGroup({ identity, groupName: issuerName });
    const issuersStore = getIssuerDetailStore({ identity, issuerName });
    issuersStore.set(group);
  } catch (err: unknown) {
    // TODO: Handle error
  }
};
