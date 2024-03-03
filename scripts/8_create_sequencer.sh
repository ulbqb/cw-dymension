#!/bin/bash

source ./env
SEQUENCER_HOME=$HOME/.sequencer
SEQUENCER_ACCOUNT=sequencer

COMMON_OPT="--from $SEQUENCER_ACCOUNT --gas-prices 0.025$HUB_DENOM --gas auto --gas-adjustment 1.5 --node $HUB_RPC_URL --keyring-backend test --chain-id $HUB_CHAIN_ID --home $SEQUENCER_HOME -b sync -o json -y"

SEQ_ADDR=$($HUB_CHAIND keys show $SEQUENCER_ACCOUNT --keyring-backend test -a --home $SEQUENCER_HOME)
PUB_KEY=$(rollapp-evm dymint show-sequencer)
MSG=$(cat reqs/rollapp_create_sequencer.json \
    | jq ".sequencer.create_sequencer.creator=\"$SEQ_ADDR\"" \
    | jq ".sequencer.create_sequencer.dymint_pub_key=$PUB_KEY" \
    | jq ".sequencer.create_sequencer.rollapp_id=\"$ROLLAPP_CHAIN_ID\"" \
    | jq ".sequencer.create_sequencer.description.moniker=\"$ROLLAPP_MONIKER\"")

{
    IFS=$'\n' read -r -d '' RESULT_STDERR;
    IFS=$'\n' read -r -d '' RESULT_STDOUT;
} < <((printf '\0%s\0' "$($HUB_CHAIND tx wasm execute $HUB_CONTRACT "$MSG" $COMMON_OPT)" 1>&2) 2>&1)

if [[ $RESULT_STDOUT == "" ]]; then
    echo "Unexpected Error"
    echo $RESULT_STDERR
    exit 1
fi

TXHASH=$(echo $RESULT_STDOUT | jq -r .txhash)
echo "txhash: $TXHASH"
