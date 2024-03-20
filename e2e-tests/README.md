# End To End Tests

Project to run e2e tests against the VC Playground projects meta_issuer and relying party.

## Install Dependencies

Install all the dependencies by running `npm ci`.

## Environment Requirements

To run the project you need the following projects.

* Local replica started with `dfx`.
* Deploy canisters to the local replica.
* TEMPORARY: Run the dev server of the issuer in `issuer/frontend`.

## Run Tests

Run the tests with the command `npm run test:e2e`.

You can change the browsers and whether it's headless (default no) in the [playwright.config.ts](./playwright.config.ts) file.
