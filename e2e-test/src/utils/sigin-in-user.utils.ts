import { expect, type BrowserContext, type Page } from '@playwright/test';

export const signInWithNewUser = async ({
  page,
  context,
}: {
  page: Page;
  context: BrowserContext;
}) => {
  const button = page.locator('[data-tid=login-button]');
  await expect(button).toBeVisible();
  await button.click();
  
  const iiPagePromise = context.waitForEvent('page');
  const iiPage = await iiPagePromise;
  await expect(iiPage).toHaveTitle('Internet Identity');

  await iiPage.locator('#registerButton').click();
  await iiPage.locator('[data-action=construct-identity]').click();

  await iiPage.locator('input#captchaInput').fill('a');
  await iiPage.locator('#confirmRegisterButton').click();

  await iiPage.locator('#displayUserContinue').click();

  await iiPage.waitForEvent('close');
  expect(iiPage.isClosed()).toBe(true);
};
