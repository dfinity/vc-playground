import { Actor, HttpAgent, type ActorSubclass, type Identity } from '@dfinity/agent';
import { idlFactory as issuerIdlFactory } from '../../declarations/meta_issuer';
import { idlFactory as rpIdlFactory } from '../../declarations/rp';
import type { _SERVICE as IssuerService } from '../../declarations/meta_issuer/meta_issuer.did';
import type { _SERVICE as RpService } from '../../declarations/rp/rp.did';
import { getOwnCanisterId } from './canister-id.utils';
import type { IDL } from '@dfinity/candid';

export const createActor = async <T>({
  canisterId,
  agent,
  fetchRootKey,
  idlFactory,
}: {
  canisterId: string;
  agent: HttpAgent;
  fetchRootKey: boolean;
  idlFactory: IDL.InterfaceFactory;
}): Promise<ActorSubclass<T>> => {
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
  return Actor.createActor<T>(idlFactory, {
    agent,
    canisterId,
  });
};

// Record of identity principals to meta issuer canister actors
const metaIssuerCanister: Record<string, ActorSubclass<IssuerService>> = {};
export const getMetaIssuerCanister = async (identity?: Identity) => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!metaIssuerCanister[identityPrincipal]) {
    const agent = new HttpAgent({ host: import.meta.env.VITE_HOST, identity });
    metaIssuerCanister[identityPrincipal] = await createActor<IssuerService>({
      canisterId: import.meta.env.VITE_ISSUER_CANISTER_ID,
      agent,
      fetchRootKey: import.meta.env.VITE_FETCH_ROOT_KEY === 'true',
      idlFactory: issuerIdlFactory,
    });
  }
  return metaIssuerCanister[identityPrincipal];
};

// Record of identity principals to meta issuer canister actors
const rpCanister: Record<string, ActorSubclass<RpService>> = {};
export const getRpCanister = async (identity?: Identity) => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!rpCanister[identityPrincipal]) {
    const agent = new HttpAgent({ host: import.meta.env.VITE_HOST, identity });
    rpCanister[identityPrincipal] = await createActor<RpService>({
      canisterId: getOwnCanisterId(),
      agent,
      fetchRootKey: import.meta.env.VITE_FETCH_ROOT_KEY === 'true',
      idlFactory: rpIdlFactory,
    });
  }
  return rpCanister[identityPrincipal];
};
