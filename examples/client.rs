use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction
};
use std::str::FromStr;

#[tokio::main()]
async fn main() {
    let program_id = Pubkey::from_str("3PyR6M4EXq6UpEgZft58NDHbupqx38PkKrLCCTTSRbWp").unwrap();

    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();

    let airdrop_amount = 1_000_000_000;

    let signature = client
        .request_airdrop(&payer.pubkey(), airdrop_amount)
        .expect("Failed to request airdrop");

    loop {
        let confirmed = client.confirm_transaction(&signature).unwrap();
        if confirmed {
            break;
        }
    }

    let instruction = Instruction::new_with_borsh(
        program_id, 
        &(),
        vec![]
    );
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction signature: {}", signature),
        Err(error) => eprintln!("Error sending transaction: {}", error),
    }
}
