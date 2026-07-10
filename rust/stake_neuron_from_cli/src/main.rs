use clap::Parser;
use stake_neuron_from_cli::{stake_neuron, StakeNeuronArgs};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Stake ICP to create an NNS Governance neuron via the two-step manual flow")]
struct Args {
    /// Path to the identity PEM file (Ed25519; use ic-agent's "ring" feature for Secp256k1)
    #[arg(short, long)]
    identity: PathBuf,

    /// URL of the IC replica (default: icp-cli local network)
    #[arg(short, long, default_value = "http://localhost:8000")]
    url: String,

    /// Amount to stake in e8s — minimum 100_000_000 (1 ICP); the 10_000 e8s transfer fee is charged on top
    #[arg(short, long, default_value = "100000000")]
    amount_e8s: u64,

    /// Nonce — a unique u64 per neuron; controls the staking subaccount
    #[arg(short, long, default_value = "0")]
    nonce: u64,

    /// Print the staking subaccount and exit without transferring any ICP.
    /// Use this to verify the destination before committing funds.
    #[arg(long)]
    compute_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.compute_only {
        use stake_neuron_from_cli::compute_neuron_staking_subaccount;
        let pem = std::fs::read(&args.identity)?;
        let identity: Box<dyn ic_agent::Identity> =
            if let Ok(id) = ic_agent::identity::BasicIdentity::from_pem(&pem) {
                Box::new(id)
            } else {
                Box::new(ic_agent::identity::Secp256k1Identity::from_pem(&pem)?)
            };
        let controller = identity.sender()?;
        let sub = compute_neuron_staking_subaccount(controller, args.nonce);
        let hex: String = sub.0.iter().map(|b| format!("{b:02x}")).collect();
        println!("Controller : {controller}");
        println!("Nonce      : {}", args.nonce);
        println!("Subaccount : {hex}");
        return Ok(());
    }
    stake_neuron(StakeNeuronArgs {
        identity_path: args.identity,
        ic_url: args.url,
        amount_e8s: args.amount_e8s,
        nonce: args.nonce,
    })
    .await?;
    Ok(())
}
