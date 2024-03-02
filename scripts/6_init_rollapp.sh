#!/bin/bash

source ./env
SEQUENCER_HOME=$HOME/.sequencer

tmp=$(mktemp)

EXECUTABLE="rollapp-evm"
ROLLAPP_CHAIN_DIR="$HOME/.rollapp_evm"

set_denom() {
  denom=$1
  jq --arg denom $denom '.app_state.mint.params.mint_denom = $denom' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
  jq --arg denom $denom '.app_state.staking.params.bond_denom = $denom' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
  jq --arg denom $denom '.app_state.gov.deposit_params.min_deposit[0].denom = $denom' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
  
  jq --arg denom $denom '.app_state.evm.params.evm_denom = $denom' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
  jq --arg denom $denom '.app_state.claims.params.claims_denom = $denom' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
}

set_EVM_params() {
  jq '.consensus_params["block"]["max_gas"] = "40000000"' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
  jq '.app_state["feemarket"]["params"]["no_base_fee"] = true' "$GENESIS_FILE" > "$tmp" && mv "$tmp" "$GENESIS_FILE"
}

# ---------------------------- initial parameters ---------------------------- #
# Assuming 1,000,000 tokens
#half is staked
TOKEN_AMOUNT="1000000000000000000000000$ROLLAPP_DENOM"
STAKING_AMOUNT="500000000000000000000000$ROLLAPP_DENOM"

CONFIG_DIRECTORY="$ROLLAPP_CHAIN_DIR/config"
GENESIS_FILE="$CONFIG_DIRECTORY/genesis.json"
TENDERMINT_CONFIG_FILE="$CONFIG_DIRECTORY/config.toml"
APP_CONFIG_FILE="$CONFIG_DIRECTORY/app.toml"
DYMINT_CONFIG_FILE="$CONFIG_DIRECTORY/dymint.toml"

# --------------------------------- run init --------------------------------- #
if ! command -v $EXECUTABLE >/dev/null; then
  echo "$EXECUTABLE does not exist"
  echo "please run make install"
  exit 1
fi

if [ -z "$ROLLAPP_CHAIN_ID" ]; then
  echo "ROLLAPP_CHAIN_ID is not set"
  exit 1
fi

# Verify that a genesis file doesn't exists for the dymension chain
if [ -f "$GENESIS_FILE" ]; then
  printf "\n======================================================================================================\n"
  echo "A genesis file already exists [$GENESIS_FILE]. building the chain will delete all previous chain data. continue? (y/n)"
  printf "\n======================================================================================================\n"
  read -r answer
  if [ "$answer" != "${answer#[Yy]}" ]; then
    rm -rf "$ROLLAPP_CHAIN_DIR"
  else
    exit 1
  fi
fi

# ------------------------------- init rollapp ------------------------------- #
$EXECUTABLE init "$ROLLAPP_MONIKER" --chain-id "$ROLLAPP_CHAIN_ID"  --home "$ROLLAPP_CHAIN_DIR"

# ------------------------------- client config ------------------------------ #
$EXECUTABLE config chain-id "$ROLLAPP_CHAIN_ID"  --home "$ROLLAPP_CHAIN_DIR"

# -------------------------------- app config -------------------------------- #
perl -pi -e "s|^minimum-gas-prices *= .*|minimum-gas-prices = \"0$ROLLAPP_DENOM\"|" $APP_CONFIG_FILE
set_denom "$ROLLAPP_DENOM"
set_EVM_params

# ------------------------------- dymint config ------------------------------ #
perl -pi -e "s|^settlement_layer *= .*|settlement_layer = \"cosmwasm\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^rollapp_id *= .*|rollapp_id = \"$ROLLAPP_CHAIN_ID\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^node_address *= .*|node_address = \"$HUB_RPC_URL\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^gas_prices *= .*|gas_prices = \"0.025$HUB_DENOM\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^address_prefix *= .*|address_prefix = \"$HUB_ADDR_PREFIX\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^contract *= .*|contract = \"$HUB_CONTRACT\"|" $DYMINT_CONFIG_FILE
perl -pi -e "s|^keyring_home_dir *= .*|keyring_home_dir = \"$SEQUENCER_HOME\"|" $DYMINT_CONFIG_FILE

# ------------------------------- dymint config ------------------------------ #
perl -pi -e "s|laddr = \"tcp://127.0.0.1:26657\"|laddr = \"$ROLLAPP_LADDR\"|" $TENDERMINT_CONFIG_FILE

# --------------------- adding keys and genesis accounts --------------------- #
#local genesis account
$EXECUTABLE keys add "$ROLLAPP_KEY_NAME" --keyring-backend test --home "$ROLLAPP_CHAIN_DIR"
$EXECUTABLE add-genesis-account "$ROLLAPP_KEY_NAME" "$TOKEN_AMOUNT" --keyring-backend test --home "$ROLLAPP_CHAIN_DIR"
$EXECUTABLE gentx_seq --pubkey "$($EXECUTABLE dymint show-sequencer)" --from "$ROLLAPP_KEY_NAME" --keyring-backend test --home "$ROLLAPP_CHAIN_DIR"

echo "Do you want to include staker on genesis? (Y/n) "
read -r answer
if [ ! "$answer" != "${answer#[Nn]}" ] ;then
  $EXECUTABLE gentx "$ROLLAPP_KEY_NAME" "$STAKING_AMOUNT" --chain-id "$ROLLAPP_CHAIN_ID" --keyring-backend test --home "$ROLLAPP_CHAIN_DIR"
  $EXECUTABLE collect-gentxs --home "$ROLLAPP_CHAIN_DIR"
fi

$EXECUTABLE validate-genesis --home "$ROLLAPP_CHAIN_DIR"
