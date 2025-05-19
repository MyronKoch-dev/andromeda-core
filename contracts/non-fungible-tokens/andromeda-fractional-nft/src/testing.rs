#[cfg(test)]
mod tests {
    use super::super::contract::{instantiate, ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg { kernel_address: "kernel".into(), owner: None };
        instantiate(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_not_implemented() {
        let deps = mock_dependencies();
        let _env = mock_env();
        let _info = mock_info("creator", &[]);
        let _msg = ExecuteMsg::CreateFractionalizedAsset {
            asset: super::super::contract::AssetParams {
                cw721_code_id: 1,
                name: "Asset".into(),
                symbol: "AST".into(),
                token_uri: None,
                token_id: "1".into(),
            },
            fractions: super::super::contract::FractionalParams {
                cw20_code_id: 1,
                supply: 1000,
                name: "Fraction".into(),
                symbol: "FRAC".into(),
                decimals: 6,
            },
        };
        // execute not implemented yet
    }
}
