// src/p2p_full.rs
use libp2p::{
    gossipsub, identity, mdns, noise, swarm::{SwarmBuilder, SwarmEvent}, tcp, yamux, PeerId, Swarm, Multiaddr
};
use libp2p::gossipsub::{IdentTopic, MessageAuthenticity};
use crate::{block::Block, snn_core::SNNCore, bus::MessageBus};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn start_full_p2p(snn: Arc<SNNCore>, bus: MessageBus) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("P2P Node Started | Peer ID: {}", local_peer_id);

    let transport = tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let topic = IdentTopic::new("pappap-ai-chain-snn-2025");

    let mut swarm = SwarmBuilder::with_tokio_executor(
        transport,
        libp2p::swarm::Behaviour::new(
            gossipsub::Behaviour::new(
                MessageAuthenticity::Signed(local_key.clone()),
                gossipsub::ConfigBuilder::default().build().unwrap(),
            ).unwrap(),
            mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap(),
        ),
        local_peer_id,
    ).build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    swarm.behaviour_mut().0.subscribe(&topic).unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Gửi block mới ra mạng
    let bus_tx = bus.sender();
    tokio::spawn(async move {
        while let Some((_, data)) = rx.recv().await {
            let _ = swarm.behaviour_mut().0.publish(topic.clone(), data);
        }
    });

    // Nhận block từ mạng
    tokio::spawn(async move {
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {}", address);
                        }
                        SwarmEvent::Behaviour(libp2p::swarm::BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            message, ..
                        })) => {
                            if let Ok(block) = serde_json::from_slice::<Block>(&message.data) {
                                println!("Received Block #{} from network | Spike Score: {:.3}", block.index, block.spike_score);
                                let _ = bus_tx.send(("block_received".to_string(), message.data.to_vec()));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}