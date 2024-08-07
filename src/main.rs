use elephant::prelude::*;
use libp2p::{futures::StreamExt, identity::Keypair, swarm::SwarmEvent, Multiaddr, SwarmBuilder};
use std::{any::Any, error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let id = Keypair::generate_ed25519();

    let mut swarm = SwarmBuilder::with_existing_identity(id)
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| libp2p::ping::Behaviour::default())?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    if let Some(port) = std::env::args().nth(1) {
        swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{port}").parse()?)?;
    }

    if let Some(addr) = std::env::args().nth(2) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {address:#?}")
            }
            SwarmEvent::Behaviour(event) => {
                println!("Received event: {:#?}", event);

                let mut contract = Epht0::new(event.peer.to_base58(), 100);

                let _ = contract.execute(20);
            }

            _ => {}
        }
    }

    Ok(())
}
