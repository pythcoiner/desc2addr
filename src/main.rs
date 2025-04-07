use clap::Parser;
use desc2addr::{Desc2Addr, Desc2AddrError};

#[derive(Parser, Debug)]
// A tool for generate receive/change addresses from a Liana descriptor
struct Cli {
    /// The descriptor to generate address from
    #[arg(short, long)]
    descriptor: String,
    /// Starting index
    #[arg(short, long)]
    start: Option<u32>,
    /// Address count
    #[arg(short, long)]
    count: Option<u32>,
}

fn main() -> Result<(), Desc2AddrError> {
    let cli = Cli::parse();

    let desc2addr = Desc2Addr::new(
        cli.descriptor,
        cli.start.unwrap_or(0),
        cli.count.unwrap_or(10),
    )?;

    let output = desc2addr.run()?;

    println!("{}", output);
    Ok(())
}
