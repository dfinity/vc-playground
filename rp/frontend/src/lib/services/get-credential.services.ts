import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { decodeJwt } from 'jose';

const II_URL = import.meta.env.VITE_INTERNET_IDENTITY_URL;
const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

let iiWindow: Window | null = null;

export const getCredential = async ({
  groupName,
  identity,
}: {
  groupName: string;
  identity: Identity | undefined | null;
}): Promise<string | null> => {
  if (isNullish(identity)) {
    return null;
  }
  return new Promise<string | null>((resolve) => {
    const handleFlowFinished = (evnt: MessageEvent) => {
      try {
        console.log('Flow finished', evnt.data);
        // Make the presentation presentable
        const verifiablePresentation = evnt.data?.result?.verifiablePresentation;
        if (verifiablePresentation === undefined) {
          console.error('No verifiable presentation found');
          resolve(null);
        }

        /* eslint-disable-next-line */
        const ver = decodeJwt(verifiablePresentation) as any;
        const creds = ver.vp.verifiableCredential;
        const [alias, credential] = creds.map((cred: string) =>
          JSON.stringify(decodeJwt(cred), null, 2)
        );
        console.log('Alias:', alias);
        console.log('Credential:', credential);
        const resultElement = document.getElementById('vc-result');
        if (resultElement) {
          resultElement.innerText = `Alias: ${alias}\nCredential: ${credential}`;
        }
      } catch (error) {
        resolve(null);
      } finally {
        iiWindow?.close();
        window.removeEventListener('message', handleFlowFinished);
        resolve('approved');
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
    iiWindow = window.open(url, '_blank');
  });
};
