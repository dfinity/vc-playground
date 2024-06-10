import { validateCredentials } from '$lib/api/validateCredentials.api';
import { credentialsStore } from '$lib/stores/credentials.store';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { popupCenter } from '$lib/utils/login-popup.utils';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import {
  requestVerifiablePresentation,
  type VerifiablePresentationResponse,
} from '@dfinity/verifiable-credentials/request-verifiable-presentation';
import type { CredentialSpec } from '../../declarations/rp/rp.did';
import { credentialArgsToObj } from '$lib/utils/credential-args-to-obj.utils';

const ISSUER_ORIGIN = import.meta.env.VITE_ISSUER_ORIGIN;
const ISSUER_CANISTER_ID = import.meta.env.VITE_ISSUER_CANISTER_ID;

export const loadCredential = async ({
  groupName,
  owner,
  credentialSpec,
  identity,
}: {
  groupName: string;
  owner: Principal;
  credentialSpec: CredentialSpec;
  identity: Identity | undefined | null;
}): Promise<null> => {
  if (isNullish(identity)) {
    return null;
  }
  console.info('Loading credential for', credentialSpec.credential_type, owner.toText());
  return new Promise<null>((resolve) => {
    requestVerifiablePresentation({
      onSuccess: async (verifiablePresentation: VerifiablePresentationResponse) => {
        if ('Err' in verifiablePresentation) {
          console.warn(verifiablePresentation.Err);
          credentialsStore.setCredential({
            groupName,
            owner,
            credentialSpec,
            hasCredential: false,
          });
          resolve(null);
          return;
        }
        const isValidCredential = await validateCredentials({
          identity,
          requestParams: {
            vcSubject: identity.getPrincipal(),
            // URL used by meta-issuer in the issued verifiable credentials (hard-coded in meta-issuer)
            issuerOrigin: 'https://metaissuer.vc/',
            issuerCanisterId: Principal.fromText(ISSUER_CANISTER_ID),
            vpJwt: verifiablePresentation.Ok,
            credentialSpec,
          },
        });
        credentialsStore.setCredential({
          groupName,
          owner,
          credentialSpec,
          hasCredential: isValidCredential,
        });
        resolve(null);
      },
      onError() {
        credentialsStore.setCredential({
          groupName,
          owner,
          credentialSpec,
          hasCredential: false,
        });
        resolve(null);
      },
      issuerData: {
        origin: ISSUER_ORIGIN,
        canisterId: Principal.fromText(ISSUER_CANISTER_ID),
      },
      credentialData: {
        credentialSpec: {
          credentialType: credentialSpec.credential_type,
          arguments: {
            // We need to add the owner so that the issuer can identify which is the issuer
            // from which the credential is being requested.
            owner: owner.toText(),
            ...credentialArgsToObj(credentialSpec),
          },
        },
        credentialSubject: identity.getPrincipal(),
      },
      windowOpenerFeatures: popupCenter(),
      identityProvider: new URL(import.meta.env.VITE_INTERNET_IDENTITY_URL),
    });
  });
};
