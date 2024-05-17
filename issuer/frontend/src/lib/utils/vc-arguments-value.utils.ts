import type { VcArguments } from '../../declarations/meta_issuer.did';

export const vcArgumentsValue = (vcArguments: [VcArguments] | []): string | number | undefined => {
  if (vcArguments.length === 0) return undefined;
  // Only support one argument for now
  const [_key, argumentValue] = vcArguments[0][0];
  if ('Int' in argumentValue) {
    return argumentValue.Int;
  }
  if ('String' in argumentValue) {
    return argumentValue.String;
  }
  throw new Error(`Unsupported argument value type: ${JSON.stringify(argumentValue)}`);
};
