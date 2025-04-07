use std::str::FromStr;

use liana::{
    descriptors::{LianaDescError, LianaDescriptor},
    miniscript::bitcoin::Network,
};

#[derive(Debug)]
pub struct Desc2Addr {
    descriptor: LianaDescriptor,
    start: u32,
    count: u32,
}

#[derive(Debug)]
pub enum Desc2AddrError {
    InvalidDescriptor(LianaDescError),
    InvalidCount(String),
    InvalidNetwork(String),
}

impl std::fmt::Display for Desc2AddrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Desc2AddrError::InvalidDescriptor(msg) => write!(f, "Invalid descriptor: {}", msg),
            Desc2AddrError::InvalidCount(msg) => write!(f, "Invalid count: {}", msg),
            Desc2AddrError::InvalidNetwork(msg) => write!(f, "Invalid network: {}", msg),
        }
    }
}

impl Desc2Addr {
    /// Create a new Desc2Addr instance
    pub fn new(desc: String, start: u32, count: u32) -> Result<Self, Desc2AddrError> {
        let descriptor =
            LianaDescriptor::from_str(&desc).map_err(Desc2AddrError::InvalidDescriptor)?;

        Ok(Self {
            descriptor,
            start,
            count,
        })
    }

    /// Generate the addresses
    pub fn run(&self) -> Result<String, Desc2AddrError> {
        let change = self.descriptor.change_descriptor();

        let receive = self.descriptor.receive_descriptor();

        let network = if self.descriptor.all_xpubs_net_is(Network::Bitcoin) {
            Network::Bitcoin
        } else if self.descriptor.all_xpubs_net_is(Network::Testnet) {
            Network::Testnet
        } else if self.descriptor.all_xpubs_net_is(Network::Regtest) {
            Network::Regtest
        } else {
            return Err(Desc2AddrError::InvalidNetwork(
                "All xpubs must be on the same network".to_string(),
            ));
        };

        // Check if the count is greater than Start
        if self.count < self.start {
            return Err(Desc2AddrError::InvalidCount(
                "Count must be greater than Start".to_string(),
            ));
        }

        // Create a new Secp256k1 instance
        let secp = liana::miniscript::bitcoin::secp256k1::Secp256k1::new();
        let mut output = "[".to_string();
        for i in self.start..=self.start + self.count - 1 {
            output = format!(
                "{}\n    {{\n        \"index\": {},\n        \"receive\": \"{}\",\n        \"change\": \"{}\"\n    }},",
                output,
                i,
                receive.derive(i.into(), &secp).address(network),
                change.derive(i.into(), &secp).address(network)
            );
        }

        // remove the last ','
        let _del = output.pop();
        output = format!("{}\n]", output);

        Ok(output)
    }
}
