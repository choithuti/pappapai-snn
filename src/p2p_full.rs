use libp2p::{
    gossipsub, identity, noise, swarm::{Swarm, SwarmEvent}, tcp, yamux, PeerId,
    gossipsub::{IdentTopic, MessageAuthenticity, AllowAllSubscriptionFilter, IdentityTransform},
    Transport,
};
use crate::{block::Block, bus::MessageBus};
use std::sync::Arc;
use futures_util::StreamExt;

pub async fn start_full_p2p(_snn: Arc<crate::snn_core::SNNCore>, bus: MessageBus) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("P2P Node Started | Peer ID: {}", local_peer_id);

    let transport = tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let topic = IdentTopic::new("pappap-ai-chain-snn-2025");

    let gossip_behaviour = gossipsub::Behaviour::<IdentityTransform, AllowAllSubscriptionFilter>::new(
        MessageAuthenticity::Signed(local_key.clone()),
        gossipsub::ConfigBuilder::default().build().unwrap(),
    ).unwrap();

    let mut swarm = Swarm::new(
        transport,
        gossip_behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor(),
    );

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    swarm.behaviour_mut().subscribe(&topic).unwrap();

    let bus_tx = bus.sender();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {}", address);
                        }
                        SwarmEvent::Behaviour(gossipsub::Event::Message { message, .. }) => {
                            if let Ok(block) = serde_json::from_slice::<Block>(&message.data) {
                                println!("Received Block #{} | Spike Score: {:.3}", block.index, block.spike_score);
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
