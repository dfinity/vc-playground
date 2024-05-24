import type { CredentialSpec } from '../../declarations/rp/rp.did';

export const credentialArgsToObj = (args: CredentialSpec): Record<string, string | number> => {
  const credArgs = args.arguments[0];
  if (!credArgs) {
    return {};
  }
  return credArgs.reduce(
    (acc, [key, value]) => {
      acc[key] = 'String' in value ? value.String : value.Int;
      return acc;
    },
    {} as Record<string, string | number>
  );
};
