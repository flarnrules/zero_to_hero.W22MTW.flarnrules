use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
// Add a map data structure to store multiple items
use cw_storage_plus::{Item, Map};

// Rename global state as config, as this is the configuration of the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct config {
    // Create an administrator
    pub admin: Addr

}

// Create the poll struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    // Three components to a poll - creator, questions, options
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64),
}

// Create the Ballot struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// A map with a String key and a poll value.
// The key will be a UUID generated clientside.
pub const POLLS: Map<String, Poll> = Map::new("polls");

// A map for ballot storage
pub const BALLOTS: map<(Addr, String), Ballot> = Map::new("ballots");

