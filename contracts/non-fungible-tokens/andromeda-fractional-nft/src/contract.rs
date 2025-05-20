use andromeda_fungible_tokens::cw20::InstantiateMsg as Cw20InstantiateMsg;
use andromeda_non_fungible_tokens::cw721::InstantiateMsg as Cw721InstantiateMsg;
use andromeda_std::{
    ado_base::{InstantiateMsg as BaseInstantiateMsg, MigrateMsg},
    ado_contract::ADOContract,
    amp::AndrAddr,
    andr_execute_fn,
    common::context::ExecuteContext,
    error::ContractError,
};
use cosmwasm_std::{
    entry_point, to_json_binary, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw20::Cw20Coin;
use cosmwasm_schema::cw_serde;

const CONTRACT_NAME: &str = "crates.io:andromeda-fractional-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct InstantiateMsg {
    pub kernel_address: String,
    pub owner: Option<String>,
}

#[cw_serde]
pub struct FractionalParams {
    pub cw20_code_id: u64,
    pub supply: u128,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct AssetParams {
    pub cw721_code_id: u64,
    pub name: String,
    pub symbol: String,
    pub token_uri: Option<String>,
    pub token_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateFractionalizedAsset {
        asset: AssetParams,
        fractions: FractionalParams,
    },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract = ADOContract::default();
    let resp = contract.instantiate(
        deps.storage,
        env,
        deps.api,
        &deps.querier,
        info,
        BaseInstantiateMsg {
            ado_type: CONTRACT_NAME.to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    Ok(resp)
}

#[andr_execute_fn]
pub fn execute(ctx: ExecuteContext, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateFractionalizedAsset { asset, fractions } => {
            execute_create_fractionalized_asset(ctx, asset, fractions)
        }
    }
}

fn execute_create_fractionalized_asset(
    mut ctx: ExecuteContext,
    asset: AssetParams,
    fractions: FractionalParams,
) -> Result<Response, ContractError> {
    let ExecuteContext { deps, env, info, .. } = &mut ctx;

    let kernel_address = ADOContract::default().get_kernel_address(deps.storage)?;

    let cw721_msg = Cw721InstantiateMsg {
        name: asset.name,
        symbol: asset.symbol,
        minter: AndrAddr::from_string(env.contract.address.to_string()),
        kernel_address: kernel_address.to_string(),
        owner: Some(info.sender.to_string()),
    };

    let cw20_msg = Cw20InstantiateMsg {
        name: fractions.name,
        symbol: fractions.symbol,
        decimals: fractions.decimals,
        initial_balances: vec![Cw20Coin {
            address: info.sender.to_string(),
            amount: Uint128::from(fractions.supply),
        }],
        mint: None,
        marketing: None,
        kernel_address: kernel_address.to_string(),
        owner: Some(info.sender.to_string()),
    };

    let cw721_inst = WasmMsg::Instantiate {
        admin: Some(info.sender.to_string()),
        code_id: asset.cw721_code_id,
        msg: to_json_binary(&cw721_msg)?,
        funds: vec![],
        label: "fractional-nft".to_string(),
    };

    let cw20_inst = WasmMsg::Instantiate {
        admin: Some(info.sender.to_string()),
        code_id: fractions.cw20_code_id,
        msg: to_json_binary(&cw20_msg)?,
        funds: vec![],
        label: "fractional-cw20".to_string(),
    };

    Ok(Response::new()
        .add_message(cw721_inst)
        .add_message(cw20_inst)
        .add_attribute("action", "create_fractionalized_asset"))
}
