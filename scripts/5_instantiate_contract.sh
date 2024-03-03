#!/bin/bash

source ./env
DEPLOYER_HOME=$HOME/.deployer
DEPLOYER_ACCOUNT=deployer

COMMON_OPT="--from $DEPLOYER_ACCOUNT --gas-prices 0.025$HUB_DENOM --gas auto --gas-adjustment 1.5 --node $HUB_RPC_URL --keyring-backend test --chain-id $HUB_CHAIN_ID --home $DEPLOYER_HOME -b sync -o json -y"

{
    IFS=$'\n' read -r -d '' RESULT_STDERR;
    IFS=$'\n' read -r -d '' RESULT_STDOUT;
} < <((printf '\0%s\0' "$($HUB_CHAIND tx wasm instantiate $HUB_CODE_ID "$(cat reqs/instantiate.json)" --label "Dymension Engine" --no-admin $COMMON_OPT)" 1>&2) 2>&1)

if [[ $RESULT_STDOUT == "" ]]; then
    echo "Unexpected Error"
    echo $RESULT_STDERR
    exit 1
fi

TXHASH=$(echo $RESULT_STDOUT | jq -r .txhash)

sleep 10
{
    IFS=$'\n' read -r -d '' RESULT_STDERR;
    IFS=$'\n' read -r -d '' RESULT_STDOUT;
} < <((printf '\0%s\0' "$($HUB_CHAIND q tx $TXHASH --node $HUB_RPC_URL -o json)" 1>&2) 2>&1)

if [[ $RESULT_STDOUT == "" ]]; then
    echo "Unexpected Error"
    echo $RESULT_STDERR
    exit 1
fi

CONTRACT=$(echo $RESULT_STDOUT | jq '.logs[0].events[] | select (.type == "instantiate").attributes[] | select (.key == "_contract_address").value' -r)

echo "txhash: $TXHASH"
echo "contract: $CONTRACT"
echo "Please set contract address to env"
