import type { CredentialSpec } from '../../declarations/rp/rp.did';

export const credentialSpecPredicate = (
  credentialSpec: CredentialSpec
): string | number | undefined => {
  const vcArgument = credentialSpec.arguments[0];
  if (vcArgument === undefined || vcArgument.length === 0) return undefined;
  // Only support one argument for now
  const [_key, argumentValue] = vcArgument[0];
  if ('Int' in argumentValue) {
    return argumentValue.Int;
  }
  if ('String' in argumentValue) {
    return argumentValue.String;
  }
  throw new Error(`Unsupported argument value type: ${JSON.stringify(argumentValue)}`);
};
