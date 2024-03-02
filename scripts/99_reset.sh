#!/bin/bash

pkill -xf 'bash endblocker.sh'
pkill rollapp-evm

rm -rf $HOME/.deployer
rm -rf $HOME/.sequencer
rm -rf $HOME/.endblocker
rm -rf $HOME/.rollapp_evm
rm -rf logs
