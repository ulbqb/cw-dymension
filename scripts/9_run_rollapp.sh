#!/bin/bash

mkdir -p logs
rollapp-evm start > ./logs/rollapp-evm.log 2>&1 &
