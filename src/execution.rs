use std::str::FromStr;

use anyhow::Result;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::transports::http::Http;
use alloy::network::{Ethereum, EthereumWallet};
use alloy::network::primitives::BlockTransactionsKind;
use alloy::rpc::types::eth::BlockNumberOrTag;
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;

pub struct Execution {
    provider: RootProvider<Http<reqwest::Client>, Ethereum>,
}

impl Execution {
    pub fn new(rpc_url: &str, private_key: &str) -> Result<Self> {
        let signer = PrivateKeySigner::from_str(private_key)?;
        let wallet = EthereumWallet::new(signer);
        // Build the provider and immediately convert it to RootProvider
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse().unwrap());
        Ok(Self { provider: provider.root().clone() })
    }

    pub fn get_provider(&self) -> RootProvider<Http<reqwest::Client>, Ethereum> {
        self.provider.clone()
    }

    pub async fn get_latest_block_number(&self) -> Result<u64> {
        let latest_block = self.provider.get_block_number().await?;
        Ok(latest_block)
    }

    pub async fn get_block_miner_by_number(&self, block_num: u64) -> Result<Option<Address>> {
        let block = self.provider.get_block_by_number(
            BlockNumberOrTag::Number(block_num),
            BlockTransactionsKind::Hashes
        ).await?;
        
        Ok(block.map(|b| b.header.beneficiary))
    }

    pub async fn get_block_timestamp_by_number(&self, block_num: u64) -> Result<Option<u64>> {
        let block = self.provider.get_block_by_number(
            BlockNumberOrTag::Number(block_num),
            BlockTransactionsKind::Hashes
        ).await?;

        Ok(block.map(|b| b.header.timestamp))
    }
}
