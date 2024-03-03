#!/bin/bash

source ./env

# deployer config
export DEPLOYER_HOME=$HOME/.deployer
export DEPLOYER_ACCOUNT=deployer

# sequencer config
export SEQUENCER_HOME=$HOME/.sequencer
export SEQUENCER_ACCOUNT=sequencer

# endblocker config
export ENDBLOCKER_HOME=$HOME/.endblocker
export ENDBLOCKER_ACCOUNT=endblocker

$HUB_CHAIND keys add $DEPLOYER_ACCOUNT --keyring-backend test --home $DEPLOYER_HOME
$HUB_CHAIND keys add $SEQUENCER_ACCOUNT --keyring-backend test --algo secp256k1 --home $SEQUENCER_HOME
$HUB_CHAIND keys add $ENDBLOCKER_ACCOUNT --keyring-backend test --home $ENDBLOCKER_HOME

curl --header "Content-Type: application/json" \
    --request POST \
    --data '{"denom":"'$HUB_DENOM'","address":"'$($HUB_CHAIND keys show $DEPLOYER_ACCOUNT --keyring-backend test -a --home $DEPLOYER_HOME)'"}' \
    $HUB_FAUCE_URL/credit &> /dev/null

curl --header "Content-Type: application/json" \
    --request POST \
    --data '{"denom":"'$HUB_DENOM'","address":"'$($HUB_CHAIND keys show $SEQUENCER_ACCOUNT --keyring-backend test -a --home $SEQUENCER_HOME)'"}' \
    $HUB_FAUCE_URL/credit &> /dev/null

curl --header "Content-Type: application/json" \
    --request POST \
    --data '{"denom":"'$HUB_DENOM'","address":"'$($HUB_CHAIND keys show $ENDBLOCKER_ACCOUNT --keyring-backend test -a --home $ENDBLOCKER_HOME)'"}' \
    $HUB_FAUCE_URL/credit &> /dev/null
