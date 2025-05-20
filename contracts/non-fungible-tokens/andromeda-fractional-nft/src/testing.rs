#[cfg(test)]
mod tests {
    use super::super::contract::{
        execute,
        instantiate,
        AssetParams,
        ExecuteMsg,
        FractionalParams,
        InstantiateMsg,
    };
    use andromeda_std::common::context::ExecuteContext;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, CosmosMsg, WasmMsg};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg { kernel_address: "kernel".into(), owner: None };
        instantiate(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_create_fractionalized_asset() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let instantiate_msg = InstantiateMsg { kernel_address: "kernel".into(), owner: None };
        instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();

        let exec_msg = ExecuteMsg::CreateFractionalizedAsset {
            asset: AssetParams {
                cw721_code_id: 1,
                name: "Asset".into(),
                symbol: "AST".into(),
                token_uri: None,
                token_id: "1".into(),
            },
            fractions: FractionalParams {
                cw20_code_id: 2,
                supply: 1000,
                name: "Fraction".into(),
                symbol: "FRAC".into(),
                decimals: 6,
            },
        };

        let ctx = ExecuteContext {
            deps: deps.as_mut(),
            env,
            info,
            querier: &deps.querier,
        };
        let res = execute(ctx, exec_msg).unwrap();

        assert_eq!(res.attributes, vec![attr("action", "create_fractionalized_asset")]);
        assert_eq!(res.messages.len(), 2);

        // Verify first message is cw721 instantiate
        if let CosmosMsg::Wasm(WasmMsg::Instantiate { code_id, .. }) = &res.messages[0].msg {
            assert_eq!(*code_id, 1);
        } else {
            panic!("expected WasmMsg::Instantiate for cw721");
        }

        if let CosmosMsg::Wasm(WasmMsg::Instantiate { code_id, .. }) = &res.messages[1].msg {
            assert_eq!(*code_id, 2);
        } else {
            panic!("expected WasmMsg::Instantiate for cw20");
        }
    }
}
