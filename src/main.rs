// src/main.rs – PHIÊN BẢN GENESIS NODE CÔNG KHAI
mod chain;
mod snn_core;
mod auto_learn;
mod bus;
mod crypto;
mod transaction;
mod block;
mod p2p_full;
mod config;

use std::sync::Arc;
use bus::MessageBus;
use config::Config;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();
    let config = Config::load();

    println!("PAPPAP AI CHAIN SNN GENESIS NODE – MADE IN VIỆT NAM");
    println!("Node ID      : {}", config.node_id);
    println!("Public IP    : {}:{}", config.public_ip, config.port);
    println!("High Neuron  : {} ({} neurons)", 
             config.high_neuron_mode, 
             if config.high_neuron_mode { "1.126.720+" } else { "112.384" });
    println!("Status       : SEED NODE ONLINE – BROADCASTING TO THE WORLD\n");

    let snn = Arc::new(SNNCore::new_with_config(&config));
    let bus = MessageBus::new();

    // BẬT FULL P2P LÀM SEED NODE
    p2p_full::start_full_p2p(snn.clone(), bus.clone(), &config).await;

    // Khởi động chain
    chain::PappapChain::new(bus.clone(), &config).await.run().await;
}