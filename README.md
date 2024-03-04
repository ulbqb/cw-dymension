# CW Dymension

This is a engine contract (like x/rollapp and x/sequencer) of [Dymension](https://dymension.xyz/) rollapp.

## Demo on Neutron Testnet (Pion)

This demo uses a pre-deployed contract on Neutron testnet so you don't need to deploy the contract. 

- Code ID: [3171](https://neutron.celat.one/pion-1/codes/3171/info)
- Contract Address: [neutron1fvlf4r7evmj8zryxgujdslnnfv4ywxu54pf68jy77hyvz6dlr8dscz7yc2](https://neutron.celat.one/pion-1/contracts/neutron1fvlf4r7evmj8zryxgujdslnnfv4ywxu54pf68jy77hyvz6dlr8dscz7yc2)

### Install Binaries

Install `neutrond`.

```shell
$ git clone git@github.com:neutron-org/neutron.git
$ cd neutron
$ git checkout v2.0.1
$ make install
$ neutrond version
2.0.1
```

Install `rollapp-evm`.

```shell
$ git clone git@github.com:ulbqb/rollapp-evm.git
$ cd rollapp-evm
$ git checkout 81405c59acb59285054cacc33a3f2932fccf83c2
$ make install
$ rollapp-evm version
1cf5795-cw-81405c59acb59285054cacc33a3f2932fccf83c2
```

### Setup Enviroiment

Copy env file for Neutron testnet and generate rollapp id.

```shell
$ cd scripts
$ cp envs/pion.env env
$ perl -pi -e "s|^export ROLLAPP_CHAIN_ID=.*|export ROLLAPP_CHAIN_ID=$(tr -dc a-z </dev/urandom | head -c 8; echo)_1000-1|" env
```

### Setup Account

Generate an account for sequencer.

```shell
$ neutrond keys add sequencer --keyring-backend test --home $HOME/.sequencer
```

Refer to [Neutron Doc](https://docs.neutron.org/neutron/faq/#where-is-the-testnet-faucet) and get the test token. And then check your balance.

```shell
$ neutrond q bank balances $(neutrond keys show sequencer --keyring-backend test --home $HOME/.sequencer -a) --node https://rpc-palvus.pion-1.ntrn.tech:443
balances:
- amount: "1000000"
  denom: untrn
pagination:
  next_key: null
  total: "0"
```

### Run Rollapp

Setup rollapp and run.

```shell
$ cd scripts
$ bash 6_init_rollapp.sh
$ bash 7_create_rollapp.sh
$ bash 8_create_sequencer.sh
$ bash 9_run_rollapp.sh
$ tail -f logs/rollapp-evm.log
```

## Dependecies

- https://github.com/ulbqb/cw-dymension
    - Equivalent to dymension@v3.0.0
    - Directories
        - cosmwasm - Engine contract
        - endblocker - Finalizer for rollapp blocks
        - scripts - Demo scripts
- https://github.com/ulbqb/rollapp-evm/tree/1cf5795-cw
    - Replace ulbqb/dymint with dymensionxyz/dymint
- https://github.com/ulbqb/dymint/tree/v0.6.1-beta-cw
    - Add cosmwasm clinet
- https://github.com/ulbqb/cosmwasm/tree/v1.1.9_plus
    - Update serde-json-wasm
