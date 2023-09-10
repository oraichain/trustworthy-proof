use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub base_ipfs: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub base_ipfs: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Addr,
        base_ipfs: Option<String>,
    },
    UpdateProof {
        // in base 58
        report_hash: String,
        ai_provider: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Proof)]
    Proof { report_hash: String },
    #[returns(ConfigResponse)]
    Config {},
}
