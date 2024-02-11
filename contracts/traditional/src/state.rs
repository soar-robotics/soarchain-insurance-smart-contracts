use cosmwasm_std::{Order, StdResult, Storage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

use crate::liabilitypolicy::LiabilityPolicy;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub insurer: String,
    pub denom: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const POLICES: Map<&str, LiabilityPolicy> = Map::new("policy");

/// This returns the list of ids for all registered escrows
pub fn all_policy_insured_parties(storage: &dyn Storage) -> StdResult<Vec<String>> {
    POLICES
        .keys(storage, None, None, Order::Ascending)
        .collect()
}