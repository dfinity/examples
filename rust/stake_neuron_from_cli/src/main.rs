use clap::Parser;
use ic_agent::{Agent, Identity};
use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
use std::path::PathBuf;
use std::io::Cursor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the private key file (PEM format)
    #[arg(short, long)]
    identity: PathBuf,

    /// URL of the IC replica
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Loading identity from: {}", args.identity.display());
    println!("Connecting to IC at: {}", args.url);

    // Try to load the identity from the PEM file
    let identity = load_identity(&args.identity).await?;
    
    println!("Identity principal: {}", identity.sender()?);

    // Create the IC agent
    let agent = Agent::builder()
        .with_url(&args.url)
        .with_identity(identity)
        .build()
        .expect("Failed to create IC agent");

    // Fetch root key for local development (DO NOT do this in production)
    agent.fetch_root_key().await?;
    
    println!("Successfully created IC agent!");
    println!("Agent is ready to interact with canisters");

    Ok(())
}

async fn load_identity(path: &PathBuf) -> Result<Box<dyn Identity>, Box<dyn std::error::Error>> {
    println!("Attempting to load identity from: {}", path.display());
    
    if !path.exists() {
        return Err(format!("Identity file not found: {}", path.display()).into());
    }

    let key_content = std::fs::read_to_string(path)?;
    
    // Try BasicIdentity (Ed25519) first
    if let Ok(identity) = BasicIdentity::from_pem(Cursor::new(key_content.as_bytes())) {
        println!("Loaded Ed25519 identity");
        return Ok(Box::new(identity));
    }
    
    // Try Secp256k1Identity 
    if let Ok(identity) = Secp256k1Identity::from_pem(Cursor::new(key_content.as_bytes())) {
        println!("Loaded secp256k1 identity");
        return Ok(Box::new(identity));
    }
    
    Err("Failed to parse identity file as either Ed25519 or secp256k1".into())
}
