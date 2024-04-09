export const errorToString = (err: unknown): string => {
  if (typeof err !== 'object' || err === null) {
    return 'An unknown error occurred';
  }
  if ('Internal' in err) {
    return `Internal error: ${err.Internal}`;
  } else if ('NotFound' in err) {
    return `Not found: ${err.NotFound}`;
  } else if ('NotAuthorized' in err) {
    return `Not authorized: ${err.NotAuthorized}`;
  } else if ('AlreadyExists' in err) {
    return `Already exists: ${err.AlreadyExists}`;
  } else if ('NotAuthenticated' in err) {
    return `Not authenticated: ${err.NotAuthenticated}`;
  } else if ('message' in err) {
    return (err as Error).message;
  } else {
    return 'An unknown error occurred';
  }
};
