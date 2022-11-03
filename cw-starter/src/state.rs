use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// Create Config struct with poll admin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
   pub admin: Addr
}

// Create Poll struct with creator, question, and options
// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
   pub creator: Addr,
   pub question: String,
   pub options: Vec<(String, u64)>,
}

// Create Ballot struct with option
// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
   pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// A map with a String key and Poll value
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");

// A map with a Addr key and string value
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");

