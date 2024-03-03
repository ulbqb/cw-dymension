#! /bin/bash

COMMON_OPT="--from $ACCOUNT --gas-prices 0.025$DENOM --gas auto --gas-adjustment 1.5 --node $RPC_URL --keyring-backend test --chain-id $CHAIN_ID --home $HOME_PATH -b sync -o json -y"

while true
do
    {
        IFS=$'\n' read -r -d '' RESULT_STDERR;
        IFS=$'\n' read -r -d '' RESULT_STDOUT;
    } < <((printf '\0%s\0' "$($CHAIND tx wasm execute $CONTRACT "{\"system\":{\"end_blocks\":{\"num\":null}}}" $COMMON_OPT)" 1>&2) 2>&1)
    
    CODE=$(echo $RESULT_STDOUT | jq -r .code)
    if [[ $CODE == "0" ]]; then
        TXHASH=$(echo $RESULT_STDOUT | jq -r .txhash)
        echo "[$(date --rfc-3339=seconds)] Finalized blocks, txhash: $TXHASH"
    elif [[ $RESULT_STDERR == *"No block to finalize"* ]]; then
        echo "[$(date --rfc-3339=seconds)] No block to finalize"
    else
        echo "[$(date --rfc-3339=seconds)] Unexpected Error, err: $RESULT_STDERR"
        exit 1
    fi
    sleep 10
done
