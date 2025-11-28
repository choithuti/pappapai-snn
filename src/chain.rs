use crate::{snn_core::SNNCore, bus::MessageBus, block::Block};
use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::io::{self, Write};

pub struct PappapChain {
    snn: Arc<SNNCore>,
    bus: MessageBus,
    height: Arc<RwLock<u64>>,
    last_hash: Arc<RwLock<String>>,
}

impl PappapChain {
    pub async fn new(bus: MessageBus) -> Self {
        let snn = Arc::new(SNNCore::new());
        println!("SNN Initialized: {} neurons | Power: {:.1}", snn.neuron_count().await, snn.power().await);
        io::stdout().flush().unwrap();

        Self {
            snn,
            bus,
            height: Arc::new(RwLock::new(0)),
            last_hash: Arc::new(RwLock::new("genesis-pappap-2025-vn".to_string())),
        }
    }

    pub async fn run(self) {
        // Clone riêng cho HTTP và Producer
        let height_http = self.height.clone();
        let snn_http    = self.snn.clone();

        let height_prod = self.height.clone();
        let last_hash   = self.last_hash.clone();
        let snn_prod    = self.snn.clone();
        let bus_tx      = self.bus.sender();

        // HTTP SERVER
        let server = HttpServer::new(move || {
            let height = height_http.clone();
            let snn    = snn_http.clone();

            App::new()
                .app_data(web::Data::new(snn))
                // Cả 2 route dùng cùng 1 closure, nên chỉ clone 1 lần
                .route("/", web::get().to({
                    let height = height.clone();
                    move || {
                        let height = height.clone();
                        async move {
                            let h = height.read().await;
                            HttpResponse::Ok()
                                .content_type("text/html; charset=utf-8")
                                .body(format!(
                                    "<h1 style='text-align:center;margin-top:20%;font-family:Arial;color:#0066cc'>
                                    PAPPAP AI CHAIN<br>GENESIS NODE VIETNAM<br><br>
                                    Block Height: {}<br>Spike Power: 0.999+</h1>", *h
                                ))
                        }
                    }
                }))
                .route("/api/health", web::get().to({
                    let height = height.clone();
                    move || {
                        let height = height.clone();
                        async move {
                            HttpResponse::Ok().json(json!({"status":"LIVE","height":*height.read().await,"node":"choithuti_GENESIS_001"}))
                        }
                    }
                }))
                .route("/api/status", web::get().to(|| async {
                    HttpResponse::Ok().json(json!({"status":"PAPPAP LIVE","node":"choithuti_GENESIS_001"}))
                }))
        })
        .bind(("0.0.0.0", 8080))
        .expect("Port 8080 bị chiếm! Chạy: pkill -f pappap");

        println!("HTTP SERVER → http://YOUR_IP:8080");
        println!("BLOCK PRODUCER STARTED – 8s/block");
        io::stdout().flush().unwrap();

        let server_handle = server.run();

        tokio::select! {
            _ = server_handle => {}
            _ = async move {
                let interval = tokio::time::interval(tokio::time::Duration::from_secs(8));
                tokio::pin!(interval);

                loop {
                    interval.as_mut().tick().await;

                    let mut h = height_prod.write().await;
                    *h += 1;
                    drop(h);

                    let prev = last_hash.read().await.clone();
                    let spike = snn_prod.forward(1.9).await;

                    let block = Block::new(*height_prod.read().await, prev, vec![], spike, "choithuti".to_string());
                    *last_hash.write().await = block.hash.clone();

                    println!("Mined Block #{} | Spike: {:.3} | Hash: {}", block.index, block.spike_score, &block.hash[..8]);
                    io::stdout().flush().unwrap();

                    let _ = bus_tx.send(("block".to_string(), serde_json::to_vec(&block).unwrap()));
                }
            } => {}
        }
    }
}
