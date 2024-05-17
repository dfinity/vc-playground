import type { CredentialSpec } from '../../declarations/meta_issuer.did';

export const inputTypeCredentialSpec = (spec: CredentialSpec): 'text' | 'number' => {
  if (spec.arguments.length > 1 || spec.arguments.length === 0) {
    throw new Error('Only one argument is supported');
  }
  const argument = spec.arguments[0];
  return 'Int' in argument ? 'number' : 'text';
};
