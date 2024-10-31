use clap::{Parser, Subcommand};
mod faucet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Repeatedly request funds from the Aptos testnet faucet(10 APT/DAY).
    Aptos {
        /// Address to fund
        account: String,
        /// Repetition number(Max:10)
        #[arg(default_value = "10")]
        count: Option<u8>,
    },
    /// Repeatedly request funds from the Sui dev/testnet faucet.
    Sui {
        /// Address to fund
        account: String,
        /// Repetition number(Max:5)
        #[arg(default_value = "1")]
        count: Option<u8>,
        /// Which Network
        #[arg(value_enum,default_value = "testnet")]
        network: Option<faucet::sui::SuiNetwork>,
    },
    /// Repeatedly request funds from the Klaytn testnet faucet.
    Klaytn {
        /// Address to fund
        account: String,
        /// Repetition number
        #[arg(default_value = "1")]
        count: Option<u8>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Aptos { account, count } => faucet::aptos::run(account, count).await,
        Commands::Sui {
            network,
            account,
            count,
        } => faucet::sui::run(account, count, network).await,
        Commands::Klaytn { account, count } => faucet::klaytn::run(account, count).await,
    }
}
