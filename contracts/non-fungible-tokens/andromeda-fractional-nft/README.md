# Overview

The Fractional NFT ADO allows a single NFT to be split into a user-defined number of fungible tokens. It deploys a CW721 contract representing the underlying asset and a CW20 contract that represents ownership shares.

## InstantiateMsg
- `kernel_address`: Address of the Andromeda kernel contract.
- `owner`: Optional owner of the ADO.

## ExecuteMsg
### `CreateFractionalizedAsset`
Deploys both the CW721 and CW20 contracts.
- `asset`: `AssetParams`
  - `cw721_code_id`: Code ID for the CW721 contract
  - `name`: NFT collection name
  - `symbol`: NFT collection symbol
  - `token_uri`: Optional metadata URI for the token
  - `token_id`: Token ID to mint in the newly created CW721 contract
- `fractions`: `FractionalParams`
  - `cw20_code_id`: Code ID for the CW20 contract
  - `supply`: Total supply of fractional tokens
  - `name`: Token name
  - `symbol`: Token symbol
  - `decimals`: CW20 decimals

Executing this message returns a response containing two `WasmMsg::Instantiate` messages for the CW721 and CW20 contracts. The caller receives the full supply of the newly created fractional tokens.

## Testing

Run `cargo test --workspace` to execute the unit tests. The provided test ensures that the contract produces the expected instantiate messages when creating a fractionalized asset.

