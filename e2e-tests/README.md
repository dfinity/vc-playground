# End To End Tests

Project to run e2e tests against the VC Playground projects meta_issuer and relying party.

## Install Dependencies

Install all the dependencies by running `npm ci` and `npx playwright install --with-deps`.

## Steps To Setup the Environment

1. Start local replica with `dfx start --clean`.
2. Create canisters (not deploy them yet).
```bash
dfx canister create internet_identity
dfx canister create meta_issuer
dfx canister create rp
```
3. Deploy canisters
```bash
dfx deploy --no-wallet --network local internet_identity
# From root directory
./scripts/deploy-meta-issuer.sh --dfx-network local
# From root directory
./scripts/deploy-rp.sh --dfx-network local
```

**Note: the deployment script will change the `issuer/frontend/static/.well-known/ii-alternative-origins` file. You can ignore and revert this change.**

## Run Tests

To run the tests you need a few environment variables set and passed to Playwright:

```bash
export REPLICA_SERVER_PORT=$(dfx info webserver-port)
export ISSUER_URL_TEMP="http://$(dfx canister id meta_issuer --network local).localhost:${REPLICA_SERVER_PORT}"
export RP_URL_TEMP="http://$(dfx canister id rp --network local).localhost:${REPLICA_SERVER_PORT}"
# From the same directory as this README.md
ISSUER_URL=$ISSUER_URL_TEMP RP_URL=$RP_URL_TEMP npm run test:e2e
````

You can change the browsers and whether it's headless (default no) in the [playwright.config.ts](./playwright.config.ts) file.
