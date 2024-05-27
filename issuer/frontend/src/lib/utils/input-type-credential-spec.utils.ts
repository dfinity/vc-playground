import type { CredentialSpec } from '../../declarations/meta_issuer.did';

export const inputTypeCredentialSpec = (
  spec: CredentialSpec
): 'text' | 'number' | 'countries' | undefined => {
  const specArguments = spec.arguments[0];
  if (specArguments === undefined) {
    return undefined;
  }
  // We use a `select` with all the countries for the `VerifiedResidence` credential.
  if (spec.credential_type === 'VerifiedResidence') {
    return 'countries';
  }
  if (specArguments.length > 1) {
    throw new Error('Only one argument is supported');
  }
  const [_, argument] = specArguments[0];
  return 'Int' in argument ? 'number' : 'text';
};
