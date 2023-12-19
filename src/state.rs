use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Token;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub creator: String,
    pub peer: String,
    pub tokens: Vec<Token>,
    pub ended: bool,
    pub creator_signed: bool,
    pub peer_signed: bool,
}

pub const STATE: Item<State> = Item::new("state");