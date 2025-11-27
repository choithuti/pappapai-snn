// src/wallet.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub address: String,
    pub balance: f64,
    pub staked_neurons: u64,
    pub total_power: f64,
}

pub static WALLETS: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, Wallet>>>> = 
    once_cell::sync::Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "MAPLE0276_GENESIS_001".to_string(),
            Wallet {
                address: "MAPLE0276_GENESIS_001".to_string(),
                balance: 120_001_287_644.42,
                staked_neurons: 561920,
                total_power: 561920.0 * 1.337,
            },
        );
        Arc::new(RwLock::new(map))
    });