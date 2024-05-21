import type { CredentialSpec } from '../../declarations/meta_issuer/meta_issuer.did';

export const inputTypeCredentialSpec = (spec: CredentialSpec): 'text' | 'number' | undefined => {
  const specArguments = spec.arguments[0];
  if (specArguments === undefined) {
    return undefined;
  }
  if (specArguments.length > 1) {
    throw new Error('Only one argument is supported');
  }
  const [_, argument] = specArguments[0];
  return 'Int' in argument ? 'number' : 'text';
};
