import type { CredentialSpec, VcArguments } from '../../declarations/meta_issuer.did';

export const convertToArguments = ({
  credentialSpec,
  credentialArgument,
}: {
  credentialSpec?: CredentialSpec;
  credentialArgument: number | string | undefined;
}): VcArguments | undefined => {
  if (
    credentialSpec === undefined ||
    credentialSpec.arguments.length === 0 ||
    !credentialArgument
  ) {
    return undefined;
  }
  // We have already checked that the arguments array is not empty.
  const specArguments = credentialSpec.arguments[0];
  const vcArguments: VcArguments = [];
  for (const [key, argumentValue] of specArguments) {
    if ('Int' in argumentValue) {
      const numberValue = Number(credentialArgument);
      if (Number.isNaN(numberValue)) {
        throw new Error('Expected a number');
      }
      vcArguments.push([key, { Int: numberValue }]);
    }
    if ('String' in argumentValue) {
      if (typeof credentialArgument !== 'string') {
        throw new Error('Expected a string');
      }
      vcArguments.push([key, { String: credentialArgument }]);
    }
  }
  return vcArguments;
};
