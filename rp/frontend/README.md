# Relying Party Frontend

Project to build the frontend assets for the relying party.

This project is developed using [Svelte Kit](https://kit.svelte.dev/).

## Developing

```bash
npm ci
npm run dev
```

## Troubleshooting

### Principals Don't Match In Issuer (Dev environment)

Error: You log in the relying party with the same anchor as in the issuer, but you get the error that the principals don't match.

Fix: You need to log in the issuer through the deployed canister in `<canisterId>.localhost:<replicaPort>`.
