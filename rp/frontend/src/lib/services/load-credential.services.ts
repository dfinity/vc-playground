import { validateCredentials } from '$lib/api/validateCredentials.api';
import { credentialsStore } from '$lib/stores/credentials.store';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { popupCenter } from '$lib/utils/login-popup.utils';
import { nonNullish } from '$lib/utils/non-nullish';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import type { ToastStore } from '@skeletonlabs/skeleton';

const II_URL = import.meta.env.VITE_INTERNET_IDENTITY_URL;
const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

let iiWindow: Window | null = null;
let nextFlowId = 0;

export const loadCredential = async ({
  groupName,
  owner,
  identity,
  toastStore,
}: {
  groupName: string;
  owner: Principal;
  identity: Identity | undefined | null;
  toastStore: ToastStore;
}): Promise<null> => {
  nextFlowId += 1;
  if (isNullish(identity)) {
    return null;
  }
  console.info('Loading credential for', groupName, owner.toText());
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
          derivationOrigin: import.meta.env.VITE_RP_DERIVATION_ORIGIN,
        },
      };
      window.addEventListener('message', handleFlowFinished);
      window.removeEventListener('message', handleFlowReady);
      evnt.source?.postMessage(req, { targetOrigin: evnt.origin });
    };
    const finishFlow = async (evnt: MessageEvent) => {
      try {
        if (nonNullish(evnt.data?.error)) {
          throw new Error(evnt.data.error);
        }
        // Make the presentation presentable
        const verifiablePresentation = evnt.data?.result?.verifiablePresentation;
        if (verifiablePresentation === undefined) {
          console.info('No verifiable presentation found');
          credentialsStore.setCredential({
            groupName,
            owner,
            hasCredential: false,
          });
        } else {
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
        }
      } catch (err) {
        console.error('Error verifying the credential', JSON.stringify(err));
        toastStore.trigger({
          message: `Oops! There was an error presenting the credential. Please try again. ${err}`,
          background: 'variant-filled-error',
        });
      } finally {
        iiWindow?.close();
        window.removeEventListener('message', handleFlowFinished);
        resolve(null);
      }
    };
    const handleFlowFinished = (evnt: MessageEvent) => {
      console.info('Message received in the finished flow handler', evnt);
      if (evnt.data?.method === 'vc-flow-ready') {
        startFlow(evnt);
      } else if (evnt.data?.id === String(nextFlowId)) {
        finishFlow(evnt);
      }
    };
    const handleFlowReady = (evnt: MessageEvent) => {
      console.info('Message received in the init flow handler', evnt);
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
