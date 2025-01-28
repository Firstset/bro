use anyhow::{Result, anyhow};
use reqwest::Client;
use crate::types::ProofResponse;

pub struct Consensus {
    cl_rpc_url: String,
    client: Client,
}

impl Consensus {
    pub fn new(cl_rpc_url: &str) -> Self {
        Self {
            cl_rpc_url: cl_rpc_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_proof_data(&self, timestamp: u64) -> Result<ProofResponse> {
        let url = format!("{}/bkit/v1/proof/block_proposer/t{}", self.cl_rpc_url, timestamp);

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to GET from CL: {:?}", e))?;

        if !resp.status().is_success() {
            return Err(anyhow!("CL responded with non-200 status: {}", resp.status()));
        }

        let proof = resp.json::<ProofResponse>().await?;
        Ok(proof)
    }
}

