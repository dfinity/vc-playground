import type { Principal } from '@dfinity/principal';
import type { ArgumentValue, CredentialSpec } from '../../declarations/rp/rp.did';

//
/**
 * This function fills the value in the first argument with the predicate and appends the owner as the second argument if present.
 *
 * For exampe:
 * - Given the credential spec: { arguments: [ [ { countryName: '<countryName>' } ] ], credentialType: "VerifiedResidence" }
 * - And the owner: P(1)
 * - And the predicate: 'Switzerland'
 *
 * The return value will be: { arguments: [ [ { countryName: 'Switzerland', owner: P(0) } ] ], credentialType: "VerifiedResidence" }
 */
export const createContentCredentialSpec = ({
  owner,
  predicate,
  credentialSpec,
}: {
  owner?: Principal;
  predicate?: string | number;
  credentialSpec: CredentialSpec;
}): CredentialSpec | undefined => {
  const credentialArguments = [];
  const specArguments = credentialSpec.arguments[0];
  if (specArguments !== undefined) {
    if (specArguments.length > 1) {
      throw new Error('Only one argument is supported');
    }
    const [key, _] = specArguments[0];
    if (predicate !== undefined) {
      const predicateArgument: ArgumentValue =
        typeof predicate === 'string' ? { String: predicate } : { Int: predicate };
      credentialArguments.push([key, predicateArgument] as [string, ArgumentValue]);
    }
  }
  if (owner !== undefined) {
    credentialArguments.push(['owner', { String: owner.toText() }] as [string, ArgumentValue]);
  }
  return {
    ...credentialSpec,
    arguments: [credentialArguments],
  };
};
