import { browser } from '$app/environment';

const USER_THEME = 'modern';
const ISSUER_THEME = 'wintry';
const LANDING_THEME = 'skeleton';
type Role = 'issuer' | 'user' | 'visitor';
const themeMapper: Record<Role, string> = {
  issuer: ISSUER_THEME,
  user: USER_THEME,
  visitor: LANDING_THEME,
};

export const setTheme = (role: Role) => {
  if (browser) {
    document.body.dataset.theme = themeMapper[role];
  }
};
