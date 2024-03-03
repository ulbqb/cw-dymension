# CW Workspace

This is a workspace to develop CosmWasm contracts for Finschia.

## How To Use

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate).

```
cargo install cargo-generate
```

Generate a workspace using a following command.

```
cargo generate --git https://github.com/Finschia/cw-workspace.git
```

Generate a contract template using a following command in the workspace directory.

```
cargo generate \
    --git https://github.com/Finschia/cw-workspace.git contracts \
    --destination $PWD/contracts
```

## Contract Option

- minimal - This is an empty contract. Please refer to [original repository](https://github.com/osmosis-labs/cw-minimal-template/tree/2c05d77b0c8fd0f44cc5c35f971263bc4b8e6419) for more information. 
- cw20 - This is a cw20-base contract. Please refer to [original repository](https://github.com/CosmWasm/cw-plus/tree/v1.1.0/contracts/cw20-base) for more information.
- cw721 - This is a cw721-base contract. Please refer to [original respository](https://github.com/CosmWasm/cw-nfts/tree/v0.16.0/contracts/cw721-base) for more information.
