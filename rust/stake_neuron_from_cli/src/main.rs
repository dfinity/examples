use clap::Parser;
use std::path::PathBuf;
use stake_neuron_from_cli::{stake_neuron, StakeNeuronArgs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the private key file (PEM format)
    #[arg(short, long)]
    identity: PathBuf,

    /// URL of the IC replica
    #[arg(short, long, default_value = "http://127.0.0.1:4943")]
    url: String,
    
    /// Amount of ICP to stake (in e8s, default: 1 ICP + fees = 100_010_000 e8s)
    #[arg(short, long, default_value = "100010000")]
    amount: u64,
    
    /// Nonce for the neuron staking subaccount
    #[arg(short, long, default_value = "0")]
    nonce: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let Args { identity, url, amount, nonce }= args;

    let stake_args = StakeNeuronArgs {
        identity_path: identity,
        ic_url: url,
        amount,
        nonce,
    };

    let _result = stake_neuron(stake_args).await?;
    Ok(())
}