use libp2p::{
    futures::StreamExt, identity::Keypair, swarm::SwarmEvent, Multiaddr, PeerId, SwarmBuilder,
};
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let id: Keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(&id.public());
    let store = libp2p_kad::store::MemoryStore::new(peer_id);
    log::info!("Peer id: {:?}", peer_id);
    let mut swarm = SwarmBuilder::with_existing_identity(id.clone())
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| libp2p_kad::Behaviour::new(peer_id, store))?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    for i in 60000..62000 {
        let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", i).parse()?;

        swarm.dial(addr)?;
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Listening on {:?}", address);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::InboundRequest { request, .. }) => {
                log::info!("Received request: {:?}", request);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::RoutablePeer { peer, address }) => {
                log::info!("Routable peer: {:?} {:?}", peer, address);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::ModeChanged { new_mode }) => {
                log::info!("Mode changed: {:?}", new_mode);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::UnroutablePeer { peer }) => {
                log::info!("Unroutable peer: {:?}", peer);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::PendingRoutablePeer { peer, address }) => {
                log::info!("Pending routable peer: {:?} {:?}", peer, address);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::RoutingUpdated {
                peer,
                is_new_peer,
                addresses,
                bucket_range,
                old_peer,
            }) => {
                log::info!(
                    "Routing updated: {:?} {:?} {:?} {:?} {:?}",
                    peer,
                    is_new_peer,
                    addresses,
                    bucket_range,
                    old_peer
                );
            }
            SwarmEvent::Dialing {
                peer_id,
                connection_id,
            } => {
                log::info!("Dialing: {:?} {:?}", peer_id, connection_id);
            }
            SwarmEvent::Behaviour(libp2p_kad::Event::OutboundQueryProgressed {
                id,
                result,
                stats,
                step,
            }) => {
                log::info!(
                    "Outbound query progressed: {:?} {:?} {:?} {:?}",
                    id,
                    result,
                    stats,
                    step
                );
            }

            SwarmEvent::IncomingConnection {
                connection_id,
                local_addr,
                send_back_addr,
            } => {
                log::info!(
                    "Incoming connection: {:?} {:?} {:?}",
                    connection_id,
                    local_addr,
                    send_back_addr
                );
            }
            SwarmEvent::ConnectionEstablished {
                peer_id,
                connection_id,
                endpoint,
                num_established,
                concurrent_dial_errors,
                established_in,
            } => {
                log::info!(
                    "Connection established: {:?} {:?} {:?} {:?} {:?} {:?}",
                    peer_id,
                    connection_id,
                    endpoint,
                    num_established,
                    concurrent_dial_errors,
                    established_in
                );
            }

            _ => {}
        }
    }
}
