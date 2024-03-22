import { Actor, HttpAgent, type ActorSubclass, type Identity } from '@dfinity/agent';
import { idlFactory as issuerIdlFactory } from "../../declarations/meta_issuer";
import type { _SERVICE } from '../../declarations/meta_issuer/meta_issuer.did';
import { getOwnCanisterId } from './canister-id.utils';
import type { IDL } from '@dfinity/candid';

export const createActor = async ({
  canisterId,
  agent,
  fetchRootKey,
  idlFactory,
}: {
  canisterId: string;
  agent: HttpAgent;
  fetchRootKey: boolean;
  idlFactory: IDL.InterfaceFactory
}): Promise<ActorSubclass<_SERVICE>> => {
  // Fetch root key for certificate validation during development
  if (fetchRootKey) {
    try {
      await agent.fetchRootKey();
    } catch (err) {
      console.warn('Unable to fetch root key. Check to ensure that your local replica is running');
      console.error(err);
    }
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId,
  });
};

// Record of identity principals to meta issuer canister actors
const metaIssuerCanister: Record<string, ActorSubclass<_SERVICE>> = {};
export const getMetaIssuerCanister = async (identity?: Identity) => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!metaIssuerCanister[identityPrincipal]) {
    const agent = new HttpAgent({ host: import.meta.env.VITE_HOST, identity });
    metaIssuerCanister[identityPrincipal] = await createActor({
      canisterId: getOwnCanisterId(),
      agent,
      fetchRootKey: import.meta.env.VITE_FETCH_ROOT_KEY === 'true',
      idlFactory: issuerIdlFactory,
    });
  }
  return metaIssuerCanister[identityPrincipal];
};

// Record of identity principals to meta issuer canister actors
const rpCanister: Record<string, ActorSubclass<_SERVICE>> = {};
export const getRpCanister = async (identity?: Identity) => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!rpCanister[identityPrincipal]) {
    const agent = new HttpAgent({ host: import.meta.env.VITE_HOST, identity });
    rpCanister[identityPrincipal] = await createActor({
      canisterId: getOwnCanisterId(),
      agent,
      fetchRootKey: import.meta.env.VITE_FETCH_ROOT_KEY === 'true',
      idlFactory: issuerIdlFactory,
    });
  }
  return rpCanister[identityPrincipal];
};
