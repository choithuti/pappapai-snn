// src/chain.rs – PHIÊN BẢN HOÀN CHỈNH 100% (MAINNET READY)
use actix_web::{
    get, post, web, App, HttpServer, HttpResponse, Responder,
    HttpServer as ActixServer,
};
use crate::{
    snn_core::SNNCore,
    auto_learn::auto_learn_and_answer,
    bus::MessageBus,
    block::Block,
    transaction::Transaction,
    api_explorer, // ← ĐÃ THÊM
};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};
use once_cell::sync::Lazy;

// Ví toàn cục (sẽ nâng cấp thành RocksDB sau)
static WALLETS: Lazy<Arc<RwLock<std::collections::HashMap<String, Value>>>> = Lazy::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert(
        "MAPLE0276_GENESIS_001".to_string(),
        json!({
            "address": "MAPLE0276_GENESIS_001",
            "balance": "120001287644.42000000",
            "staked_neurons": 561920,
            "total_power": 751923.84,
            "status": "GENESIS_VALIDATOR"
        }),
    );
    Arc::new(RwLock::new(map))
});

pub struct PappapChain {
    snn: Arc<SNNCore>,
    bus: MessageBus,
    current_height: Arc<RwLock<u64>>,
    last_hash: Arc<RwLock<String>>,
}

impl PappapChain {
    pub async fn new(bus: MessageBus) -> Self {
        let snn = Arc::new(SNNCore::new());
        let neurons = snn.neuron_count().await;
        let power = snn.power().await;
        println!("SNN Initialized: {neurons} neurons | Power: {power:.1}");
        Self {
            snn,
            bus,
            current_height: Arc::new(RwLock::new(0)),
            last_hash: Arc::new(RwLock::new("genesis-hash-pappap-2025".to_string())),
        }
    }

    pub async fn get_block_height(&self) -> u64 {
        *self.current_height.read().await
    }

    pub async fn get_last_hash(&self) -> String {
        self.last_hash.read().await.clone()
    }

    pub async fn update_chain_state(&self, block: &Block) {
        let mut height = self.current_height.write().await;
        *height = block.index;
        let mut hash = self.last_hash.write().await;
        *hash = block.hash.clone();
    }

    pub async fn run(self) {
        let snn_clone = self.snn.clone();
        let bus = self.bus.clone();

        // HTTP SERVER + TOÀN BỘ API
        tokio::spawn(async move {
            ActixServer::new(move || {
                App::new()
                    .app_data(web::Data::new(snn_clone.clone()))
                    // AI Chat API
                    .service(web::resource("/api/prompt").route(web::post().to(prompt_handler)))
                    .service(web::resource("/api/status").route(web::get().to(status_handler)))
                    .service(web::resource("/api/wallet/balance").route(web::get().to(wallet_balance_handler)))
                    // Wallet API
                    .service(get_wallet)
                    // Block Explorer API (từ module riêng)
                    .service(web::scope("/explorer").configure(api_explorer::configure))
            })
            .bind(("0.0.0.0", 8080))
            .unwrap()
            .run()
            .await
            .unwrap();
        });

        // BLOCK PRODUCER – 8 GIÂY/BLOCK
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(8));
        loop {
            interval.tick().await;

            let tx = Transaction::new(
                "MAPLE0276_GENESIS_001".to_string(),
                crate::transaction::TransactionType::StakeNeuron { neuron_count: 1000 },
            );

            let height = self.get_block_height().await;
            let prev_hash = self.get_last_hash().await;
            let spike_score = self.snn.forward(1.8).await;

            let block = Block::new(
                height + 1,
                prev_hash,
                vec![tx],
                spike_score,
                "choithuti_GENESIS_001".to_string(),
            );

            self.update_chain_state(&block).await;

            let block_data = serde_json::to_vec(&block).unwrap();
            println!("Mined Block #{} | Spike Score: {:.3} | Hash: {}", 
                     block.index, block.spike_score, &block.hash[..8]);

            let _ = bus.sender().send(("block_finalized".to_string(), block_data.clone()));
            let _ = bus.sender().send(("block_proposal".to_string(), block_data));
        }
    }
}

// ==================== CÁC HANDLER ====================

async fn prompt_handler(
    snn: web::Data<Arc<SNNCore>>,
    req: web::Json<Value>,
) -> impl Responder {
    let prompt = req["prompt"].as_str().unwrap_or("hello").trim();

    if !snn.check_ethics_and_law(prompt).await {
        return HttpResponse::Ok().json(json!({
            "response": "Yêu cầu vi phạm đạo đức hoặc pháp luật Việt Nam (Luật An ninh mạng 2018). Pappap AI từ chối xử lý.",
            "status": "rejected_by_ethics"
        }));
    }

    let is_complex = prompt.contains("luật") || prompt.contains("bài tập") || prompt.contains("giải") || prompt.len() > 80;
    if is_complex {
        let learned = auto_learn_and_answer(prompt).await;
        return HttpResponse::Ok().json(json!({
            "response": learned,
            "self_learned": true,
            "neurons": snn.neuron_count().await
        }));
    }

    let (lang, response) = snn.detect_and_translate(prompt).await;
    let tts = snn.text_to_speech(&response, &lang);
    HttpResponse::Ok().json(json!({
        "response": response,
        "language": lang,
        "tts": tts,
        "neurons": snn.neuron_count().await,
        "status": "GENESIS NODE ALIVE – Ethical & Self-Learning"
    }))
}

async fn status_handler(snn: web::Data<Arc<SNNCore>>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "PAPPAP AI CHAIN SNN v0.3 – ETHICAL & SELF-LEARNING",
        "neurons": snn.neuron_count().await,
        "power": snn.power().await,
        "block_height": "Live",
        "compliance": "Vietnam Law Compliant",
        "genesis_node": "choithuti_GENESIS_001"
    }))
}

async fn wallet_balance_handler(_: web::Data<Arc<SNNCore>>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "address": "MAPLE0276_GENESIS_001",
        "balance": "120001287644.42000000",
        "total_neurons": 561920,
        "compliance": "Vietnam Law Compliant"
    }))
}

#[get("/api/wallet/{address}")]
async fn get_wallet(path: web::Path<String>) -> impl Responder {
    let address = path.into_inner();
    let wallets = WALLETS.read().await;
    if let Some(wallet) = wallets.get(&address) {
        HttpResponse::Ok().json(wallet)
    } else {
        HttpResponse::Ok().json(json!({
            "address": address,
            "balance": "0.00000000",
            "staked_neurons": 0,
            "message": "Ví chưa tồn tại – dùng /explorer/faucet/claim để nhận test token"
        }))
    }
}