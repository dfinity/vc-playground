import { browser } from '$app/environment';

const CREDENTIALS_THEME = 'modern';
const ISSUER_THEME = 'wintry';
type Role = 'issuer' | 'credentials';
const themeMapper: Record<Role, string> = {
  issuer: CREDENTIALS_THEME,
  credentials: ISSUER_THEME,
};

export const setTheme = (role: Role) => {
  if (browser) {
    document.body.dataset.theme = themeMapper[role];
  }
};
