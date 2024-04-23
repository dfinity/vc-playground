import { nonNullish } from '$lib/utils/non-nullish';
import type { Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import type { Principal } from '@dfinity/principal';

let cachedClient: AuthClientNew | undefined = undefined;
export const getAuthClient = async () => {
  if (!cachedClient) {
    cachedClient = await AuthClientNew.create({
      identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_URL,
      derivationOrigin: import.meta.env.VITE_RP_DERIVATION_ORIGIN,
    });
  }
  return cachedClient;
};

export const resetCachedAuthClient = () => {
  cachedClient = undefined;
}

type CredentialSpec = {
  credentialType: string;
  arguments: Record<string, string | number>;
};

type IssuerData = {
  origin: string;
  canisterId?: Principal;
};

export class AuthClientNew {
  private client: AuthClient;
  private derivationOrigin: string;
  private identityProvider: string;
  /**
   * New parameters to add when creating the client
   */
  static async create(params: { derivationOrigin: string; identityProvider: string }) {
    const client = await AuthClient.create({
      idleOptions: {
        disableIdle: true,
      },
    });
    return new AuthClientNew(client, params);
  }

  constructor(client: AuthClient, params: { derivationOrigin: string; identityProvider: string }) {
    this.client = client;
    this.derivationOrigin = params.derivationOrigin;
    this.identityProvider = params.identityProvider;
  }

  /**
   * DUPLICATION OF CURRENT FUNCTIONALITY FROM AUTHCLIENT
   */
  login({
    onError,
    onSuccess,
    windowOpenerFeatures,
    maxTimeToLive,
  }: {
    onSuccess: () => void | Promise<void>;
    onError: (error?: string) => void | Promise<void>;
    windowOpenerFeatures: string | undefined;
    maxTimeToLive: bigint;
  }) {
    this.client.login({
      onSuccess,
      onError,
      windowOpenerFeatures,
      maxTimeToLive,
      derivationOrigin: this.derivationOrigin,
      identityProvider: this.identityProvider,
    });
  }

  getIdentity(): Identity {
    return this.client.getIdentity();
  }

  isAuthenticated(): Promise<boolean> {
    return this.client.isAuthenticated();
  }

  logout() {
    return this.client.logout();
  }
}

let iiWindow: Window | null = null;
let nextFlowId = 0;
/**
 * PROPOSAL OF HOW TO ADD VC FUNCTIONALITY v2
 */
export const requestCredential = ({
  onSuccess,
  onError,
  credentialSpec,
  credentialSubject,
  issuerData,
  windowOpenerFeatures,
  derivationOrigin,
  identityProvider,
}: {
  onSuccess: (verifiablePresentation: string) => void | Promise<void>;
  onError: (err?: string) => void | Promise<void>;
  credentialSpec: CredentialSpec;
  credentialSubject: Principal;
  issuerData: IssuerData;
  windowOpenerFeatures: string | undefined;
  derivationOrigin: string | undefined;
  identityProvider: string;
}) => {
  nextFlowId += 1;
  const startFlow = (evnt: MessageEvent) => {
    const req = {
      id: String(nextFlowId),
      jsonrpc: '2.0',
      method: 'request_credential',
      params: {
        issuer: issuerData,
        credentialSpec,
        credentialSubject: credentialSubject.toText(),
        derivationOrigin: derivationOrigin,
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
        // This should never happen
        onError("Couldn't get the verifiable credential");
      } else {
        onSuccess(verifiablePresentation);
      }
    } catch (err) {
      onError(`Error getting the verifiable credential: ${err}`);
    } finally {
      iiWindow?.close();
      window.removeEventListener('message', handleFlowFinished);
    }
  };
  const handleFlowFinished = (evnt: MessageEvent) => {
    if (evnt.data?.method === 'vc-flow-ready') {
      startFlow(evnt);
    } else if (evnt.data?.id === String(nextFlowId)) {
      finishFlow(evnt);
    }
  };
  const handleFlowReady = (evnt: MessageEvent) => {
    if (evnt.data?.method !== 'vc-flow-ready') {
      return;
    }
    startFlow(evnt);
  };
  window.addEventListener('message', handleFlowReady);
  const url = new URL(identityProvider);
  url.pathname = 'vc-flow/';
  iiWindow = window.open(url, '_blank', windowOpenerFeatures);
};
