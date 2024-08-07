use std::error::Error;

use libp2p::PeerId;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epht0 {
    owner: PeerId,
    balance: u64,
}

impl Epht0 {
    pub fn new(owner: PeerId, balance: u64) -> Self {
        Epht0 { owner, balance }
    }

    pub fn execute(&mut self, amount: u64) -> Result<(), Box<dyn Error>> {
        log::info!("Executing contract with amount: {}", amount);
        if self.balance < amount {
            log::error!("Insufficient funds");
            return Err("Insufficient funds".into());
        }

        log::info!("Deducting amount from balance");

        self.balance -= amount;

        Ok(())
    }
}
