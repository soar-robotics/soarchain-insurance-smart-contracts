use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

use crate::policy::Policy;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub policy_holder: String,
    pub insured_party: String,
    pub denom: String,
    pub base_rate: u64,
    pub rate_per_mile: u64,
}

pub const STATE: Item<State> = Item::new("state");
pub const POLICES: Map<&str, Policy> = Map::new("policy");

