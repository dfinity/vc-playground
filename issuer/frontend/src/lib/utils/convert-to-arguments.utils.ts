import type { CredentialSpec, VcArguments } from '../../declarations/meta_issuer.did';

export const convertToArguments = ({
  credentialSpec,
  userInput,
}: {
  credentialSpec?: CredentialSpec;
  userInput: number | string | undefined;
}): VcArguments | undefined => {
  if (credentialSpec === undefined || credentialSpec.arguments.length === 0 || !userInput) {
    return undefined;
  }
  if (credentialSpec.arguments.length > 1) {
    throw new Error('Only one argument is supported');
  }
  const specArguments = credentialSpec.arguments[0];
  if (specArguments.length > 1) {
    throw new Error('Only one argument is supported');
  }
  const [[key, argumentValue]] = specArguments;
  if ('Int' in argumentValue) {
    const numberValue = Number(userInput);
    if (Number.isNaN(numberValue)) {
      throw new Error('Expected a number');
    }
    return [[key, { Int: numberValue }]];
  }
  if ('String' in argumentValue) {
    if (typeof userInput !== 'string') {
      throw new Error('Expected a string');
    }
    return [[key, { String: userInput }]];
  }
};
