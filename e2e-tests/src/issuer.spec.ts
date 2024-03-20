import { test, expect } from '@playwright/test';
import { signInWithNewUser } from './utils/sigin-in-user.utils';

test('use registers with Internet Identity and is redirected to the home page', async ({
  page,
  context,
}) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/VC Playground/);
  expect(page.getByTestId('home-route')).not.toBeVisible();

  await signInWithNewUser({ page, context });

  expect(page.getByTestId('home-route')).toBeVisible();
});
