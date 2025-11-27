// src/transaction.rs
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TransactionType {
    Transfer { to: String, amount: u64 },
    StakeNeuron { neuron_count: u64 },
    UnstakeNeuron { neuron_count: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub tx_type: TransactionType,
    pub timestamp: u64,
    pub signature: String, // hex string (sẽ thêm ký thật sau)
}

impl Transaction {
    pub fn new(from: String, tx_type: TransactionType) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            from,
            tx_type,
            timestamp,
            signature: "pappap-genesis-sig-2025".to_string(),
        }
    }

    pub fn hash(&self) -> String {
        let data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}