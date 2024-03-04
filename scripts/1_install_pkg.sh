#!/bin/bash

cd ..

# finscia devnet and rollapp-evm
cd scripts
mkdir -p tmp
if [ ! -d "tmp/rollapp-evm" ]; then
  git clone https://github.com/ulbqb/rollapp-evm.git tmp/rollapp-evm
fi
cd tmp/rollapp-evm
git checkout 81405c59acb59285054cacc33a3f2932fccf83c2
make install
cd ../..

if [ ! -d "tmp/finschia" ]; then
  git clone https://github.com/Finschia/finschia.git tmp/finschia
fi
cd tmp/finschia
git checkout v2.0.1
make install
cd ../..

if [ ! -d "tmp/tutorials" ]; then
  git clone https://github.com/Finschia/tutorials.git tmp/tutorials
fi
cd ..

while getopts ":a" flag; do
  case $flag in
  a)
    ARCH_OPT="-arm64"
    ;;
  \?)
    # Handle invalid options
    ;;
  esac
done

exit 0

# build contracts
cd cosmwasm
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer${ARCH_OPT}:0.13.0 contracts/engine/
if [[ $ARCH_OPT == "-arm64" ]]; then
  mv artifacts/engine-aarch64.wasm artifacts/engine.wasm
fi
cd ..
