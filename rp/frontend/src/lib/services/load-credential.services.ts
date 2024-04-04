import { credentialsStore } from '$lib/stores/credentials.store';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { popupCenter } from '$lib/utils/login-popup.utils';
import { nonNullish } from '$lib/utils/non-nullish';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { decodeJwt } from 'jose';

const II_URL = import.meta.env.VITE_INTERNET_IDENTITY_URL;
const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

let iiWindow: Window | null = null;
let nextFlowId = 0;

export const loadCredential = async ({
  groupName,
  owner,
  identity,
}: {
  groupName: string;
  owner: Principal;
  identity: Identity | undefined | null;
}): Promise<null> => {
  nextFlowId += 1;
  if (isNullish(identity)) {
    return null;
  }
  return new Promise<null>((resolve) => {
    const startFlow = (evnt: MessageEvent) => {
      const principal = identity.getPrincipal().toText();
      const req = {
        id: String(nextFlowId),
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
              owner: owner.toText(),
            },
          },
          credentialSubject: principal,
        },
      };
      window.addEventListener('message', handleFlowFinished);
      window.removeEventListener('message', handleFlowReady);
      evnt.source?.postMessage(req, { targetOrigin: evnt.origin });
    };
    const finishFlow = (evnt: MessageEvent) => {
      try {
        if (nonNullish(evnt.data?.error)) {
          throw new Error(evnt.data.error);
        }
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
        console.error('Error verifying the credential', error);
      } finally {
        iiWindow?.close();
        window.removeEventListener('message', handleFlowFinished);
        resolve(null);
      }
    };
    const handleFlowFinished = (evnt: MessageEvent) => {
      console.log('in da handleFlowFinished', evnt);
      if (evnt.data?.method === 'vc-flow-ready') {
        startFlow(evnt);
      } else if (evnt.data?.id === String(nextFlowId)) {
        finishFlow(evnt);
      }
    };
    const handleFlowReady = (evnt: MessageEvent) => {
      console.log('in da handleFlowReady', evnt);
      if (evnt.data?.method !== 'vc-flow-ready') {
        return;
      }
      startFlow(evnt);
    };
    window.addEventListener('message', handleFlowReady);
    const url = new URL(II_URL);
    url.pathname = 'vc-flow/';
    iiWindow = window.open(url, '_blank', popupCenter());
  });
};
