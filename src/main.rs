use std::str::FromStr;
use std::thread;
use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// cluster type
    #[arg(short, long, value_enum, default_value_t = Mode::Devnet)]
    r#type: Mode,

    /// solana account address
    #[arg(short, long)]
    address: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum,Debug)]
enum Mode {
    /// devnet
    #[clap(alias = "d")]
    Devnet,

    /// testnet
    #[clap(alias = "t")]
    Testnet,
}

fn get_airdrop(client:&RpcClient,pk:&Pubkey) {
    let airdrop_amount = LAMPORTS_PER_SOL * 5;
    let signature = client.request_airdrop(&pk, airdrop_amount);
    match signature {
        Ok(signature) => {
            loop{
                let r = client.confirm_transaction(&signature);
                let flag = r.unwrap();
                if flag {
                    break
                }
            }
            println!("airdrop 5 sol to {} success", &pk.to_string());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn main() {
    let args = Args::parse();
    let rpc_url = match args.r#type {
        Mode::Devnet => "https://api.devnet.solana.com",
        Mode::Testnet => "https://api.testnet.solana.com",
    };
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let pk = Pubkey::from_str(&args.address).unwrap();
    loop {
        get_airdrop(&client,&pk);
        thread::sleep(Duration::from_secs(1800));
    }
}
