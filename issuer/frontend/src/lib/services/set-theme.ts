const CREDENTIALS_THEME = 'modern';
const ISSUER_THEME = 'wintry';
type Role = 'issuer' | 'credentials';
const themeMapper: Record<Role, string> = {
  issuer: ISSUER_THEME,
  credentials: CREDENTIALS_THEME,
};

export const setTheme = (role: Role) => {
  document.body.dataset.theme = themeMapper[role];
};
