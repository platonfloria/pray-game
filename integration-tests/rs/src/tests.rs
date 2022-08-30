use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

mod helpers;

const CHARACTER_WASM_FILEPATH: &str = "../../out/character.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Add tests here
    Ok(())
}
