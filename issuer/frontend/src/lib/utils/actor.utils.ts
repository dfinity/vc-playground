import { Actor, HttpAgent, type ActorSubclass, type Identity } from '@dfinity/agent';
import { idlFactory } from '../../declarations';
import type { _SERVICE } from '../../declarations/meta_issuer.did';

export const createActor = async ({
  canisterId,
  agent,
  fetchRootKey,
}: {
  canisterId: string;
  agent: HttpAgent;
  fetchRootKey: boolean;
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

let metaIssuerCaniter: ActorSubclass<_SERVICE> | undefined = undefined;
export const getMetaIssuerCanister = async (identity?: Identity) => {
  if (!metaIssuerCaniter) {
    const agent = new HttpAgent({ host: import.meta.env.VITE_HOST, identity });
    metaIssuerCaniter = await createActor({
      canisterId: import.meta.env.VITE_ISSUER_CANISTER_ID,
      agent,
      fetchRootKey: import.meta.env.VITE_FETCH_ROOT_KEY === 'true',
    });
  }
  return metaIssuerCaniter;
};
