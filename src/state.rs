use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub base_ipfs: String,
}

#[cw_serde]
pub struct Proof {
    // Created time: in seconds
    pub created_time: u64,
    // AI provider
    pub ai_provider: String,
    // Report link
    pub report_link: String,
}

// put the length bytes at the first for compatibility with legacy singleton store
pub const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");

// store temporary pair info while waiting for deployment
pub const PROOFS: Map<&[u8], Proof> = Map::new("proofs");
