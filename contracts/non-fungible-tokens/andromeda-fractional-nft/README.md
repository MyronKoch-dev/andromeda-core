# Overview

The Fractional NFT ADO allows a single non-fungible token to be created and immediately split into a user-defined number of fungible tokens. It automates deployment of a CW721 contract for the unique asset and a CW20 contract representing fractional ownership. Each fraction corresponds to a portion of the NFT and can be freely transferred like any fungible token.

This README describes the expected workflow, messages and usage for the fractionalizer contract.

## Fractionalization Workflow

1. **Instantiate** the Fractional NFT contract with the on-chain kernel address and optional owner address.
2. **Execute** `CreateFractionalizedAsset` providing two parameter sets:
   - `asset`: metadata for the underlying CW721 NFT (code ID, name, symbol, token ID and optional URI).
   - `fractions`: parameters for the CW20 token (code ID, total supply, name, symbol and decimals).
3. On execution the contract will:
   - Instantiate a new CW721 contract with the given parameters.
   - Mint a single token with the supplied `token_id` to the contract itself or the caller.
   - Instantiate a CW20 contract representing fractional shares.
   - Mint the entire `supply` of fractions to the caller.
4. The caller now holds CW20 tokens representing ownership of the NFT. The CW721 token can later be redeemed or transferred once all fractions are returned to the contract.

*Note:* The current code base only contains a skeleton implementation. Actual fractionalization logic is marked TODO in `execute` and should handle token deployment, minting and redemption.

## InstantiateMsg

```json
{
  "kernel_address": "string",
  "owner": "optional string"
}
```

- `kernel_address` – address of the aOS kernel on the target chain.
- `owner` – optional contract owner. If omitted the message sender becomes the owner.

## ExecuteMsg

### `CreateFractionalizedAsset`

```json
{
  "create_fractionalized_asset": {
    "asset": {
      "cw721_code_id": "u64",
      "name": "string",
      "symbol": "string",
      "token_uri": "optional string",
      "token_id": "string"
    },
    "fractions": {
      "cw20_code_id": "u64",
      "supply": "u128",
      "name": "string",
      "symbol": "string",
      "decimals": "u8"
    }
  }
}
```

Instantiates new CW721 and CW20 contracts and mints the specified amounts. The fractions are transferred to the sender.

## Building

Use the project build script to compile this contract into a wasm binary:

```bash
./build.sh andromeda-fractional-nft
```

This places the optimized `.wasm` in the root `artifacts` directory.

## Testing

Run all unit tests across the workspace with:

```bash
cargo test --workspace
```

End-to-end tests can be created under `tests/e2e` following the instructions in that package. Ensure the necessary artifacts are built and a local network is running before executing e2e tests.

## Further Reading

For general ADO concepts and architecture see the main [Andromeda documentation](https://docs.andromedaprotocol.io/). Additional guides on building and interacting with ADOs are found in this repository's root README.
