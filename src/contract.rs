use std::str::FromStr;

use alloy::dyn_abi::SolType;
use alloy::sol_types::sol_data::Bool;
use anyhow::Result;
use alloy::primitives::{Bytes, Address, FixedBytes};
use alloy::providers::Provider;
use alloy::transports::http::Http;
use alloy::network::{Ethereum, TransactionBuilder};
use alloy::providers::RootProvider;
use alloy::rpc::types::eth::TransactionRequest;
use alloy::sol;
use alloy::sol_types::SolCall;
use crate::types::ProofResponse;

// Define the function interface using the sol! macro
sol!(
    function isTimestampActionable(uint64 timestamp) external view returns (bool);
    function distributeFor(
        uint64 nextTimestamp,
        uint64 proposerIndex,
        bytes calldata pubkey,
        bytes32[] calldata proposerIndexProof,
        bytes32[] calldata pubkeyProof
    ) external;
);

pub struct Contract {
    address: Address,
    provider: RootProvider<Http<reqwest::Client>, Ethereum>,
}

impl Contract {
    pub fn new(address: Address, provider: RootProvider<Http<reqwest::Client>, Ethereum>) -> Self {
        Self { address, provider }
    }

    pub async fn is_timestamp_actionable(&self, timestamp: u64) -> Result<bool> {
        // Create and encode the call
        let call = isTimestampActionableCall { timestamp }.abi_encode();
        let input = Bytes::from(call);

        // Create and execute the transaction
        let tx = TransactionRequest::default()
            .with_to(self.address)
            .with_input(input);

        println!("[bro] Calling isTimestampActionable with transaction: {:?}", tx);

        let response = self.provider.call(&tx).await?;

        // Decode the response
        let result = Bool::abi_decode(&response.0, false)?;

        println!("[bro] Result: {:?}", result);

        Ok(result)
    }

    pub async fn distribute_for(&self, timestamp: u64, miner: Address, proof_data: ProofResponse) -> Result<bool> {
        // Create and encode the call
        let call = distributeForCall {
            nextTimestamp: timestamp,
            proposerIndex: proof_data.beacon_block_header.proposer_index.parse()?,
            pubkey: Bytes::from(proof_data.validator_pubkey),
            proposerIndexProof: vec![FixedBytes::<32>::from_str(&proof_data.proposer_proof).unwrap()],
            pubkeyProof: vec![FixedBytes::<32>::from_str(&proof_data.validator_pubkey_proof).unwrap()],
        }.abi_encode();
        let input = Bytes::from(call);

        // Create and send the transaction
        let tx = TransactionRequest::default()
            .with_to(self.address)
            .with_from(miner)
            .with_input(input);

        let pending_tx = self.provider.send_transaction(tx).await?;
        let receipt = pending_tx.get_receipt().await?;

        println!("Receipt: {:?}", receipt);

        Ok(receipt.status())
    }
}
