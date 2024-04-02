import { credentialsStore } from '$lib/stores/credentials.store';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { popupCenter } from '$lib/utils/login-popup.utils';
import type { Identity } from '@dfinity/agent';
import { decodeJwt } from 'jose';

const II_URL = import.meta.env.VITE_INTERNET_IDENTITY_URL;
const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

let iiWindow: Window | null = null;

export const loadCredential = async ({
  groupName,
  identity,
}: {
  groupName: string;
  identity: Identity | undefined | null;
}): Promise<null> => {
  if (isNullish(identity)) {
    return null;
  }
  return new Promise<null>((resolve) => {
    const handleFlowFinished = (evnt: MessageEvent) => {
      try {
        // Make the presentation presentable
        const verifiablePresentation = evnt.data?.result?.verifiablePresentation;
        if (verifiablePresentation === undefined) {
          console.error('No verifiable presentation found');
          credentialsStore.setCredential({
            groupName,
            hasCredential: false,
          });
        } else {
          /* eslint-disable-next-line */
          decodeJwt(verifiablePresentation) as any;
          // TODO: Validate the credential

          credentialsStore.setCredential({
            groupName,
            hasCredential: true,
          });
        }
      } catch (error) {
        credentialsStore.setCredential({
          groupName,
          hasCredential: false,
        });
      } finally {
        iiWindow?.close();
        window.removeEventListener('message', handleFlowFinished);
        resolve(null);
      }
    };
    const handleFlowReady = (evnt: MessageEvent) => {
      if (evnt.data?.method !== 'vc-flow-ready') {
        return;
      }
      const principal = identity.getPrincipal().toText();
      const req = {
        id: '1',
        jsonrpc: '2.0',
        method: 'request_credential',
        params: {
          issuer: {
            origin: ISSUER_ORIGIN,
            canisterId: ISSUER_CANISTER_ID,
          },
          credentialSpec: {
            credentialType: 'VerifiedMember',
            arguments: {
              groupName,
            },
          },
          credentialSubject: principal,
        },
      };
      window.addEventListener('message', handleFlowFinished);
      window.removeEventListener('message', handleFlowReady);
      evnt.source?.postMessage(req, { targetOrigin: evnt.origin });
    };
    window.addEventListener('message', handleFlowReady);
    const url = new URL(II_URL);
    url.pathname = 'vc-flow/';
    iiWindow = window.open(url, '_blank', popupCenter());
  });
};
