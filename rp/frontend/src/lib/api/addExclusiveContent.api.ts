import { Principal } from '@dfinity/principal';
import type { ContentData } from '../../declarations/rp/rp.did';
import type { Identity } from '@dfinity/agent';

/* eslint-disable-next-line */
export const addExclusiveContent = async ({
  url,
  issuerName,
  identity,
}: {
  url: string;
  issuerName: string;
  identity: Identity;
}): Promise<ContentData> => {
  console.log('addExclusiveContent', { url, issuerName, identity });
  // TODO: Implement addExclusiveContent
  return {
    url: 'https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    owner: Principal.fromText('aaaaa-aa'),
    credential_group_name: 'Credential test',
    created_timestamp_ns: 12345n,
  };
};
