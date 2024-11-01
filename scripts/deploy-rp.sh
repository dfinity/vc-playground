#!/usr/bin/env bash

set -euo pipefail

#########
# USAGE #
#########

function title() {
    echo "Provisioning relying party canister" >&2
}

function usage() {
    cat >&2 << EOF

Usage:
  $0 [--ii-canister-id CANISTER_ID] [--dfx-network NETWORK]

Options:
  --dfx-network NETWORK             The network to use (typically "local" or "ic"), defaults to "local"
  --ii-canister-id CANISTER_ID      The canister id of II instance, defaults to the local internet_identity canister
  --issuer-canister-id CANISTER_ID  The canister id of the issuer, defaults to the local issuer canister

EOF
}

function help() {
    cat >&2 << EOF

The issuer canister needs some information to operate correctly. This reads data
from the replica to ensure the issuer is provisioned correctly.
EOF
}

DFX_NETWORK=
II_CANISTER_ID=
ISSUER_CANISTER_ID=

while [[ $# -gt 0  ]]
do
    case "$1" in
        -h|--help)
            title
            usage
            help
            exit 0
            ;;
        --dfx-network)
            DFX_NETWORK="${2:?missing value for '--dfx-network'}"
            shift; # shift past --dfx-network & value
            shift;
            ;;
        --ii-canister-id)
            II_CANISTER_ID="${2:?missing value for '--ii-canister-id'}"
            shift; # shift past argument name & value
            shift;
            ;;
        --issuer-canister-id)
            ISSUER_CANISTER_ID="${2:?missing value for '--issuer-canister-id'}"
            shift; # shift past argument name & value
            shift;
            ;;
        *)
            echo "ERROR: unknown argument $1"
            usage
            echo
            echo "Use '$0 --help' for more information"
            exit 1
            ;;
    esac
done

# Make sure we always run from the repo's root
SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPTS_DIR/.."

# URL used by II-issuer in the id_alias-verifiable credentials (hard-coded in II)
II_VC_URL="https://identity.ic0.app/"
# URL used by meta-issuer in the issued verifiable credentials (hard-coded in meta-issuer)
ISSUER_VC_URL="https://metaissuer.vc/"
# Domain of the relying party in production
RP_PROD_URL="https://relyingparty.vc/"

DFX_NETWORK="${DFX_NETWORK:-local}"
RP_CANISTER_ID="$(dfx canister id rp --network "$DFX_NETWORK")"
II_CANISTER_ID="${II_CANISTER_ID:-$(dfx canister id internet_identity --network "$DFX_NETWORK")}"
ISSUER_CANISTER_ID="${ISSUER_CANISTER_ID:-$(dfx canister id meta_issuer --network "$DFX_NETWORK")}"

RP_DERIVATION_ORIGIN=
if [ "$DFX_NETWORK" = "mainnet" ]; then
    RP_DERIVATION_ORIGIN="$RP_PROD_URL"
    elif [ "$DFX_NETWORK" = "local" ]; then
    RP_DERIVATION_ORIGIN="http://$RP_CANISTER_ID.localhost:4943"
    else
    RP_DERIVATION_ORIGIN="https://$RP_CANISTER_ID.icp0.io"
fi

echo "Using DFX network: $DFX_NETWORK" >&2
echo "Using RP canister: $RP_CANISTER_ID" >&2
echo "Using II vc_url: $II_VC_URL" >&2
echo "Using II canister: $II_CANISTER_ID" >&2
echo "Using issuer vc_url: $ISSUER_VC_URL" >&2
echo "Using issuer canister: $ISSUER_CANISTER_ID" >&2
echo "Using derivation origin: $RP_DERIVATION_ORIGIN" >&2

# At the time of writing dfx outputs incorrect JSON with dfx ping (commas between object
# entries are missing).
# In order to read the root key we grab the array from the '"root_key": [...]' bit, the brackets
# to match what candid expects ({}), replace the commas between array entries to match
# what candid expects (semicolon) and annotate the numbers with their type (otherwise dfx assumes 'nat'
# instead of 'nat8').
rootkey_did=$(dfx ping "$DFX_NETWORK" \
    | sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
    | sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
    | sed 's/,/;/g')

echo "Parsed rootkey: ${rootkey_did:0:20}..." >&2

# Add dev server to alternative origins when deploying locally
if [ "$DFX_NETWORK" = "local" ]; then
  # Adjust issuer's .well-known/ii-alternative-origins to contain FE-hostname of local/dev deployments.
  # We had a problem with `sed` command in CI. This is a hack to make it work locally and in CI.
  mv ./rp/frontend/static/.well-known/ii-alternative-origins ./ii-alternative-origins-template
  cat ./ii-alternative-origins-template | sed "s+RP_FE_HOSTNAME_PLACEHOLDER+\"http://localhost:5173\",+g"  > ./rp/frontend/static/.well-known/ii-alternative-origins
  rm ./ii-alternative-origins-template
  else
  mv ./rp/frontend/static/.well-known/ii-alternative-origins ./ii-alternative-origins-template
  cat ./ii-alternative-origins-template | sed "s+RP_FE_HOSTNAME_PLACEHOLDER++g"  > ./rp/frontend/static/.well-known/ii-alternative-origins
  rm ./ii-alternative-origins-template
fi


dfx deploy rp --network "$DFX_NETWORK" --argument '(opt record { issuers = vec{ record{ vc_url = "'"$ISSUER_VC_URL"'"; canister_id = principal "'"$ISSUER_CANISTER_ID"'" }}; ic_root_key_der = vec '"$rootkey_did"'; ii_vc_url = "'"$II_VC_URL"'"; ii_canister_id = principal"'"$II_CANISTER_ID"'"; derivation_origin = "'"$RP_DERIVATION_ORIGIN"'" })'

# Revert changes
git checkout ./rp/frontend/static/.well-known/ii-alternative-origins