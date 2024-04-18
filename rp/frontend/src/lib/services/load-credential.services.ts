import { validateCredentials } from '$lib/api/validateCredentials.api';
import { credentialsStore } from '$lib/stores/credentials.store';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { popupCenter } from '$lib/utils/login-popup.utils';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { getAuthClient } from './auth-client.services';

const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

export const loadCredential = async ({
  groupName,
  owner,
  identity,
}: {
  groupName: string;
  owner: Principal;
  identity: Identity | undefined | null;
  toastStore: ToastStore;
}): Promise<null> => {
  if (isNullish(identity)) {
    return null;
  }
  console.info('Loading credential for', groupName, owner.toText());
  const authClient = await getAuthClient();
  return new Promise<null>((resolve) => {
    authClient.requestCredential({
      onSuccess: async (verifiablePresentation: string) => {
        const isValidCredential = await validateCredentials({
          identity,
          requestParams: {
            vcSubject: identity.getPrincipal(),
            // URL used by meta-issuer in the issued verifiable credentials (hard-coded in meta-issuer)
            issuerOrigin: 'https://metaissuer.vc/',
            issuerCanisterId: Principal.fromText(ISSUER_CANISTER_ID),
            vpJwt: verifiablePresentation,
            credentialSpec: {
              credential_type: 'VerifiedMember',
              arguments: [
                [
                  ['groupName', { String: groupName }],
                  ['owner', { String: owner.toText() }],
                ],
              ],
            },
          },
        });
        credentialsStore.setCredential({
          groupName,
          owner,
          hasCredential: isValidCredential,
        });
        resolve(null);
      },
      onError() {
        credentialsStore.setCredential({
          groupName,
          owner,
          hasCredential: false,
        });
        resolve(null);
      },
      issuerData: {
        origin: ISSUER_ORIGIN,
        canisterId: ISSUER_CANISTER_ID,
      },
      credentialSpec: {
        credentialType: 'VerifiedMember',
        arguments: {
          groupName,
          owner: owner.toText(),
        },
      },
      credentialSubject: identity.getPrincipal(),
      windowOpenerFeatures: popupCenter(),
    });
  });
};
