// src/api_explorer.rs
use actix_web::{web, HttpResponse, Responder};
use crate::{block::Block, transaction::Transaction, PappapChain};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

static FAUCET_CLAIMED: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, u64>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_blocks)
       .service(get_block_by_height)
       .service(get_transaction)
       .service(faucet_claim)
       .service(network_status);
}

#[get("/api/blocks")]
async fn get_blocks(chain: web::Data<Arc<PappapChain>>) -> impl Responder {
    let mut blocks = vec![];
    // Giả lập đọc từ chaindata (sẽ nâng cấp sau)
    for i in 1..=chain.get_block_height().await {
        blocks.push(json!({
            "height": i,
            "hash": format!("fake-hash-{:08}", i),
            "tx_count": 1,
            "spike_score": format!("{:.3}", rand::random::<f32>() * 0.3 + 0.7),
            "timestamp": chrono::Utc::now().timestamp()
        }));
    }
    HttpResponse::Ok().json(blocks)
}

#[get("/api/block/{height}")]
async fn get_block_by_height(path: web::Path<u64>, chain: web::Data<Arc<PappapChain>>) -> impl Responder {
    let height = path.into_inner();
    HttpResponse::Ok().json(json!({
        "height": height,
        "hash": format!("pappap-block-{:08}", height),
        "previous_hash": if height == 1 { "genesis" } else { format!("pappap-block-{:08}", height-1) },
        "spike_score": 0.892,
        "validator": "choithuti_GENESIS_001",
        "transactions": [
            {
                "from": "MAPLE0276_GENESIS_001",
                "type": "StakeNeuron",
                "neuron_count": 1000,
                "timestamp": chrono::Utc::now().timestamp()
            }
        ]
    }))
}

#[post("/api/faucet/claim")]
async fn faucet_claim(req: web::Json<serde_json::Value>) -> impl Responder {
    let address = req["address"].as_str().unwrap_or("");
    if address.is_empty() || address.len() < 20 {
        return HttpResponse::BadRequest().json(json!({ "error": "Invalid address" }));
    }

    let mut claimed = FAUCET_CLAIMED.write().await;
    let now = chrono::Utc::now().timestamp() as u64;
    if let Some(last) = claimed.get(address) {
        if now - last < 86400 {
            return HttpResponse::Ok().json(json!({ "error": "Faucet: chỉ được claim 1 lần/ngày" }));
        }
    }
    claimed.insert(address.to_string(), now);

    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Faucet thành công! Bạn nhận được 1000 PAPPAP + 5000 Neuron",
        "amount": "1000.00000000 PAPPAP",
        "neurons": 5000,
        "address": address
    }))
}

#[get("/api/network")]
async fn network_status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "network": "Pappap AI Chain SNN Testnet 2025",
        "total_nodes": 128,
        "total_neurons": 128_000_000,
        "block_height": 1842,
        "genesis_node": "72.61.126.190:36331",
        "status": "LIVING & SPIKING"
    }))
}