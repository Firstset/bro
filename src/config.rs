use std::env;
use anyhow::{anyhow, Result};
use dotenv::dotenv;
use alloy::primitives::Address;

#[derive(Debug, Clone)]
pub struct Config {
    pub el_rpc_url: String,
    pub cl_rpc_url: String,
    pub validator_pubkey: Address,
    pub private_key: String,
    pub rewards_contract: Address,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok(); // load .env if present

        let el_rpc_url = env::var("EL_RPC_URL")
            .map_err(|_| anyhow!("Missing EL_RPC_URL env var"))?;
        let cl_rpc_url = env::var("CL_RPC_URL")
            .map_err(|_| anyhow!("Missing CL_RPC_URL env var"))?;
        let validator_pubkey = env::var("VALIDATOR_ADDRESS")
            .map_err(|_| anyhow!("Missing VALIDATOR_ADDRESS env var"))?
            .parse()
            .map_err(|_| anyhow!("Invalid VALIDATOR_ADDRESS format"))?;
        let private_key = env::var("PRIVATE_KEY")
            .map_err(|_| anyhow!("Missing PRIVATE_KEY env var"))?;
        let rewards_contract = env::var("REWARDS_CONTRACT")
            .map_err(|_| anyhow!("Missing REWARDS_CONTRACT env var"))?
            .parse()
            .map_err(|_| anyhow!("Invalid REWARDS_CONTRACT format"))?;

        Ok(Config {
            el_rpc_url,
            cl_rpc_url,
            validator_pubkey,
            private_key,
            rewards_contract,
        })
    }
}
