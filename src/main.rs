mod chain;
mod snn_core;
mod bus;
mod block;
mod transaction;

use bus::MessageBus;

#[tokio::main]
async fn main() {
    println!("\n
    ██████╗  █████╗ ██████╗ ██████╗  █████╗ ██████╗ 
    ╚════██╗██╔══██╗██╔══██╗██╔══██╗██╔══██╗╚════██╗
     █████╔╝███████║██████╔╝██████╔╝███████║ █████╔╝
    ██╔═══╝ ██╔══██║██╔══██╗██╔══██╗██╔══██║██╔═══╝ 
    ███████╗██║  ██║██║  ██║██║  ██║██║  ██║███████╗
    PAPPAP AI CHAIN v3.0 – GENESIS NODE VIETNAM
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝    World's First Living Blockchain – 27/11/2025
    https://pappapai.xyz\n");

    let bus = MessageBus::new();
    let chain = chain::PappapChain::new(bus).await;

    // CHẠY MÃI MÃI – KHÔNG BAO GIỜ THOÁT
    chain.run().await;
}
