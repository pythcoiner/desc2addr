use clap::Parser;
use liana::{descriptors::LianaDescriptor, miniscript::bitcoin::Network};

#[derive(Parser, Debug)]
// A tool for generate receive/change addresses from a Liana descriptor
struct Cli {
    /// The descriptor to generate address from
    #[arg(short, long)]
    descriptor: LianaDescriptor,
    /// Starting index
    #[arg(short, long)]
    start: Option<u32>,
    /// Address count
    #[arg(short, long)]
    count: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    let change = cli.descriptor.change_descriptor();
    let receive = cli.descriptor.receive_descriptor();
    let start = cli.start.unwrap_or(0);
    let count = cli.count.unwrap_or(1);
    let network = if cli.descriptor.all_xpubs_net_is(Network::Bitcoin) {
        Network::Bitcoin
    } else if cli.descriptor.all_xpubs_net_is(Network::Testnet) {
        Network::Testnet
    } else {
        Network::Regtest
    };

    let secp = liana::miniscript::bitcoin::secp256k1::Secp256k1::new();

    let mut output = "[".to_string();
    for i in start..=start + count - 1 {
        output = format!(
            "{}\n    {{\n        \"index\": {},\n        \"receive\": \"{}\",\n        \"change\": \"{}\"\n    }},",
            output,
            i,
            receive.derive(i.into(), &secp).address(network),
            change.derive(i.into(), &secp).address(network),
        );
    }

    // remove the last ','
    let _del = output.pop();

    output = format!("{}\n]", output);

    println!("{}", output);
}
