import type { VcArguments } from '../../declarations/meta_issuer.did';

export const vcArgumentsValue = (vcArguments: [VcArguments] | []): string | number | undefined => {
  const vcArgument = vcArguments[0];
  if (vcArgument === undefined) return undefined;
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
