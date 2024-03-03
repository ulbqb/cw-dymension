#!/bin/bash

source ./env
export CHAIN_ID=$HUB_CHAIN_ID
export CONTRACT=$HUB_CONTRACT
export CHAIND=$HUB_CHAIND
export RPC_URL=$HUB_RPC_URL
export DENOM=$HUB_DENOM
export ACCOUNT=endblocker
export HOME_PATH=$HOME/.endblocker/

mkdir -p logs

cd ../endblocker
bash endblocker.sh > ../scripts/logs/endblocker.log 2>&1 &
