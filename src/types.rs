use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BeaconBlockHeader {
    pub proposer_index: String,
}

#[derive(Debug, Deserialize)]
pub struct ProofResponse {
    pub beacon_block_header: BeaconBlockHeader,
    pub proposer_proof: String,
    pub validator_pubkey: String,
    pub validator_pubkey_proof: String,
}