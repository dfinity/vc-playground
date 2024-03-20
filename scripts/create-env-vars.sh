#!/usr/bin/env bash

print_help() {
  cat <<-EOF

	This script creates the `.env` file and populates it with the environment variables depending on the environment.

	The scripts requires DFX_NETWORK to be set and ENV_FILE is optional.

  Example: DFX_NETWORK=local ./scripts/create-env-vars.sh
	EOF
}

test -n "$DFX_NETWORK" # Will fail if not defined.
export DFX_NETWORK

ENV_FILE=${ENV_OUTPUT_FILE:-$PWD/.env}
echo "Creating .env file at $ENV_FILE"

II_CANISTER_ID=$(dfx canister id internet_identity --network "$DFX_NETWORK")
EARLY_CANISTER_ID=$(dfx canister id meta_issuer --network "$DFX_NETWORK")

if [ "$DFX_NETWORK" = "local" ]; then
  REPLICA_SERVER_PORT=$(dfx info webserver-port)
  II_URL="http://${II_CANISTER_ID}.localhost:${REPLICA_SERVER_PORT}"
  HOST="http://localhost:${REPLICA_SERVER_PORT}"
  echo "VITE_INTERNET_IDENTITY_URL=${II_URL}" > $ENV_FILE
  echo "VITE_ISSUER_CANISTER_ID=${EARLY_CANISTER_ID}" >> $ENV_FILE
  echo "VITE_HOST=${HOST}" >> $ENV_FILE
  echo "VITE_FETCH_ROOT_KEY=true" >> $ENV_FILE
fi
if [ "$DFX_NETWORK" = "devenv_llorenc" ]; then
  II_URL="https://${II_CANISTER_ID}.llorenc-ingress.devenv.dfinity.network/"
  HOST="https://llorenc-ingress.devenv.dfinity.network"
  echo "VITE_INTERNET_IDENTITY_URL=${II_URL}" > $ENV_FILE
  echo "VITE_HOST=${HOST}" >> $ENV_FILE
  echo "VITE_FETCH_ROOT_KEY=true" >> $ENV_FILE
fi
if [ "$DFX_NETWORK" = "mainnet" ]; then
  II_URL="https://${II_CANISTER_ID}.ic0.app"
  HOST="https://icp-api.io"
  echo "VITE_INTERNET_IDENTITY_URL=${II_URL}" > $ENV_FILE
  echo "VITE_HOST=${HOST}" >> $ENV_FILE
  echo "VITE_FETCH_ROOT_KEY=false" >> $ENV_FILE
fi

