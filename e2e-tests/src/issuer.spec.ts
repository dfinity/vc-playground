import { test, expect } from '@playwright/test';
import { signInWithNewUser } from './utils/sigin-in-user.utils';

const ISSUER_URL = 'http://localhost:5173/';
const RP_URL = 'http://localhost:5174/';

test('verifieable credentials flow works end to end', async ({
  browser,
}) => {
  /**
   * CREATE ISSUER AND CREDENTIAL
   */
  const issuerContext = await browser.newContext();
  const issuerPage = await issuerContext.newPage();
  await issuerPage.goto(ISSUER_URL);
  await expect(issuerPage).toHaveTitle(/Verifiable Credentials Playground/);

  expect(issuerPage.getByTestId('issuer-center-route')).not.toBeVisible();
  // await expect(issuerPage.getByTestId("go-issuer-center")).toBeVisible();
  // await expect(issuerPage.getByTestId("go-issuer-center")).toBeEnabled();
  await issuerPage.getByTestId("go-issuer-center").click();
  await expect(issuerPage.getByTestId('issuer-center-route')).toBeVisible();

  await expect(issuerPage.getByTestId('login-button')).toBeVisible();
  const issuerAnchor = await signInWithNewUser({ page: issuerPage, context: issuerContext });
  await expect(issuerPage.getByTestId('login-button')).not.toBeVisible();

  // Wait until nickname modal to be visible.
  await expect(issuerPage.locator('[data-testid=modal]')).toBeVisible();

  // Add Issuer nickname
  const issuerName = `Issuer ${issuerAnchor}`;
  await issuerPage.locator('input[name=prompt]').fill(issuerName);
  await issuerPage.locator('button[type=submit]').click();

  await expect.poll(() => issuerPage.getByTestId('page-title').textContent()).toBe(issuerName);

  await expect(issuerPage.getByTestId('open-create-credential-modal')).toBeEnabled();
  await issuerPage.getByTestId('open-create-credential-modal').click();

  const credentialName = "Works at DFINITY";
  await issuerPage.locator("#choose-credential").selectOption(credentialName);
  await issuerPage.getByTestId("create-credential").click();

  await expect(issuerPage.getByTestId(`credentials ${issuerName} ${credentialName}`)).toBeVisible();

  /**
   * PUBLISH CONTENT REQUIRING PREVIOUS CREDENTIAL
   */
  const publisherContext = await browser.newContext();
  const publisherPage = await publisherContext.newPage();
  await publisherPage.goto(RP_URL);

  await expect(publisherPage.getByTestId("share-page")).not.toBeVisible();
  // await expect(publisherPage.getByTestId("go-publish")).toBeVisible();
  // await expect(publisherPage.getByTestId("go-publish")).toBeEnabled();
  await publisherPage.getByTestId("go-publish").click();
  await expect(publisherPage.getByTestId("share-page")).toBeVisible();
  await expect(publisherPage.getByTestId("success-message")).not.toBeVisible();

  await publisherPage.locator("#credentials").selectOption(credentialName);
  await publisherPage.locator("#issuers").selectOption(issuerName);
  await publisherPage.getByTestId("choose-image").click();
  await expect(publisherPage.getByTestId("modal")).toBeVisible();
  // Click in the first image
  await publisherPage.getByTestId("image-1").click();  
  await expect(publisherPage.getByTestId("modal")).not.toBeVisible();

  await expect(publisherPage.getByTestId("publish-button")).toBeEnabled();
  await publisherPage.getByTestId("publish-button").click();

  await expect(publisherPage.getByTestId("success-message")).toBeVisible();

  /**
   * REQUEST CREDENTIAL
   */
  const requesterContext = await browser.newContext();
  const requesterPage = await requesterContext.newPage();

  await requesterPage.goto(ISSUER_URL);
  await requesterPage.getByTestId("go-credentials").click();
  await expect(requesterPage.getByTestId("credentials-page")).toBeVisible();

  await expect(requesterPage.getByTestId('login-button')).toBeVisible();
  const requesterAnchor = await signInWithNewUser({ page: requesterPage, context: requesterContext });
  
  // Wait until nickname modal to be visible.
  await expect(requesterPage.locator('[data-testid=modal]')).toBeVisible();

  // Add User nickname
  const userName = `User ${requesterAnchor}`;
  await requesterPage.locator('input[name=prompt]').fill(userName);
  await requesterPage.locator('button[type=submit]').click();

  // Wait for nickname to be set
  await expect.poll(() => requesterPage.getByTestId('page-title').textContent()).toBe(userName);

  // Request credential
  const credentialElement = requesterPage.getByTestId(`credentials ${issuerName} ${credentialName}`);
  await expect(credentialElement).toBeVisible();
  await expect(credentialElement.locator("button")).toBeEnabled();
  await credentialElement.locator("button").click();
});
