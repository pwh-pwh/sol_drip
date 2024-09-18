use std::str::FromStr;
use std::thread;
use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, system_instruction};

use clap::{Parser, ValueEnum};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_transaction;
use solana_sdk::transaction::Transaction;

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

fn create_account()->Keypair {
    Keypair::new()
}

fn send_with_new_account(client:&RpcClient,pk: &Pubkey) {
    let new_account = Keypair::new();
    match get_airdrop(client, &new_account.pubkey()) {
        Ok(_) => {
            let hash = client.get_latest_blockhash().unwrap();
            let tr = system_instruction::transfer(&new_account.pubkey(),pk,LAMPORTS_PER_SOL /10 * 49);
            let mut transaction = Transaction::new_with_payer(&[tr], Some(&new_account.pubkey()));
            transaction.sign(&[&new_account], hash);
            let signature = client.send_and_confirm_transaction(&transaction).unwrap();
            println!("{}", signature);
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
}

fn get_airdrop(client:&RpcClient,pk:&Pubkey)->Result<(), String> {
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
            Ok(())
        }
        Err(err) => {
            println!("{:?}", err);
            Err("airdrop failed".into())
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
        send_with_new_account(&client,&pk);
        thread::sleep(Duration::from_secs(60));
    }
}
