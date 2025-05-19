use super::contract::{execute, instantiate, ExecuteMsg, InstantiateMsg};
use cw_orch::prelude::*;

pub struct FractionalNftContract(pub cw_orch::daemon::daemon::UncheckedContract);

impl<Chain: CwEnv> Deployable<Chain> for FractionalNftContract {
    fn from_code_id(code_id: u64, chain: Chain) -> Self {
        FractionalNftContract(UncheckedContract::from_id(code_id, chain))
    }

    fn wrapper(&self) -> &UncheckedContract {
        &self.0
    }
}

impl<Chain: CwEnv> Uploadable<Chain> for FractionalNftContract {}

impl<Chain: CwEnv> Instantiateable<Chain> for FractionalNftContract {
    type InstantiateMsg = InstantiateMsg;

    fn instantiate(&self, chain: Chain, msg: Self::InstantiateMsg) -> anyhow::Result<Contract<Chain>> {
        self.0.instantiate(&chain, msg, None, "fractional-nft", None)
    }
}

impl<Chain: CwEnv> Executable<Chain> for FractionalNftContract {
    type ExecuteMsg = ExecuteMsg;

    fn execute<Msg>(&self, chain: Chain, msg: Msg) -> anyhow::Result<<Chain as CwEnv>::Response>
    where
        Msg: Into<Self::ExecuteMsg>,
    {
        self.0.execute(&chain, msg, None)
    }
}
