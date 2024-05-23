import type { CredentialSpec } from '../../declarations/rp/rp.did';

/**
 * Returns true if the credential types and the arguments are equal.
 */
export const equalCredentials = (credential1: CredentialSpec, credential2: CredentialSpec) => {
  if (credential1.credential_type !== credential2.credential_type) return false;
  if (credential1.arguments.length !== credential2.arguments.length) return false;
  const args1 = credential1.arguments[0];
  const args2 = credential2.arguments[0];
  if (args1 === undefined && args2 === undefined) return true;
  if (args1 === undefined || args2 === undefined) return false;
  if (args1?.length !== args2?.length) return false;
  for (const arg1 of args1) {
    const arg2 = args2?.find(([key]) => key === arg1[0]);
    // At this point the value is `{ Int: ... }` or `{ String: ... }`.
    // Therefore, stringifying the value is safe because the order of the keys is guaranteed.
    if (JSON.stringify(arg1) !== JSON.stringify(arg2)) return false;
  }
  return true;
};
