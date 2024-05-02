# VC playground

A playground for Verifiable Credentials (VC) functionality on the Internet Computer.

This repo contains an example VC issuer and a VC relying party for demo purposes.

The goal is to demonstrate the VC flow on the IC and enable playing with the flow in the roles of an issuer or a relying party.

## Code structure

- `issuer/..` contains an implementation of an issuer dapp.
- `rp/..` contains an implementation of a relying party.
- `e2e-tests/..` contains an end to end test of a successful flow getting and proving a credential.
- `.github/..` contains Github workflows such as running the e2e test on each pull request.
- `scripts/..` contains utility scripts that help with devops.
