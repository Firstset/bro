use anyhow::Result;

mod config;
mod consensus;
mod execution;
mod contract;
mod types;

use config::Config;
use execution::Execution;
use contract::Contract;
use consensus::Consensus;

const MAX_LOOKBACK_BLOCKS: u64 = 8_191;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::from_env()?;

    println!("[bro] Starting bot with config: {:?}", cfg);

    // Setup the execution, contract, and consensus instances
    let execution = Execution::new(&cfg.el_rpc_url, &cfg.private_key)?;
    let contract = Contract::new(cfg.rewards_contract, execution.get_provider());
    let consensus = Consensus::new(&cfg.cl_rpc_url);

    // 1) Get the last checked block from the EL

    let latest_block = execution.get_latest_block_number().await?;

    let mut last_checked_block_num: u64 = latest_block - MAX_LOOKBACK_BLOCKS;
    println!("[bro] Starting from block: {}", last_checked_block_num);
    loop {
        // 2) Check if block was proposed by our validator
        let miner = match execution.get_block_miner_by_number(last_checked_block_num).await? {
            Some(miner) if miner == cfg.validator_pubkey => miner,
            Some(_) => {
                println!("[bro] Block {} was proposed by another validator", last_checked_block_num);
                last_checked_block_num += 1;
                continue;
            },
            None => {
                println!("[bro] No proposer found for block {}", last_checked_block_num);
                panic!("Proposer for block {} must exist - this is not possible", last_checked_block_num);
            }
        };

        println!("[bro] Block {} was proposed by {}, continuing...", last_checked_block_num, miner);

        // 3) Get the timestamp of the parent block (i.e. the next block to be proposed after the current one we proposed)
        let parent_block_timestamp = match execution.get_block_timestamp_by_number(last_checked_block_num + 1).await? {
            Some(timestamp) => timestamp,
            None => {
                println!("[bro] No timestamp found for block {}", last_checked_block_num + 1);
                panic!("Timestamp for block {} must exist - this is not possible", last_checked_block_num + 1);
            }
        }; 

        println!("[bro] Parent block timestamp: {}", parent_block_timestamp);

        // 4) Check whether the block is actionable (rewards not yet distributed)
        let is_actionable = contract.is_timestamp_actionable(parent_block_timestamp).await?;
        if !is_actionable {
            println!("[bro] Block {} is not actionable", last_checked_block_num);
            last_checked_block_num += 1;
            continue;
        }

        // 5) Distribute rewards (obtain proof, call contract)
        let proof_data = consensus.get_proof_data(parent_block_timestamp).await?;

        let contract = Contract::new(cfg.rewards_contract, execution.get_provider());
        let success = contract.distribute_for(parent_block_timestamp, miner, proof_data).await?;
        if !success {
            panic!("[bro] Failed to distribute rewards for block {}", last_checked_block_num);
        }

        // 6) Decide whether to sleep or keep going
        //    If we have caught up to the latest block, sleep
        //    Otherwise, keep going

        last_checked_block_num += 1;
        if last_checked_block_num >= latest_block {
            println!("[bro] Caught up to the latest block, sleeping for 10 seconds");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
        
    }
}
