use std::{error::Error, hash::Hash};

use libp2p::PeerId;
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epht0 {
    owner: String,
    balance: u64,
    address: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Event {
    contract_address: String,
    operation: &'static str,
    data: Value,
}

impl Epht0 {
    pub fn new(owner: String, balance: u64) -> Self {
        let address = PeerId::random().to_base58();
        Epht0 {
            owner,
            balance,
            address,
        }
    }

    pub fn execute(&mut self, amount: u64) -> Result<Event, Box<dyn Error>> {
        log::info!("owner {}, balance {}", self.owner, self.balance);
        if self.balance < amount {
            log::error!("Insufficient funds");
            return Err("Insufficient funds".into());
        }

        log::info!("Deducting amount from balance");
        self.balance -= amount;

        Ok(Event {
            contract_address: self.address.clone(),
            operation: "execute",
            data: serde_json::json!({
                "owner": self.owner,
                "balance": self.balance,
            }),
        })
    }
}
