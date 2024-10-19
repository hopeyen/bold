use ethers::prelude::*;
use ethers_core::k256::ecdsa::SigningKey;

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

use crate::transactor::{borrower_ops::BorrowerOperations};

pub mod borrower_ops;

/// Contracts: (contract name, contract address)
pub type ContractAddresses = HashMap<String, H160>;
/// Client with provider endpoint and a wallet
pub type ContractClient = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;


#[derive(Debug)]
pub struct TransactorConfigs {
    // batch manager pvk
    pub private_key: String,
    pub provider: String,
    pub subgraph: String,
}

impl TransactorConfigs {
    pub fn dummy() -> Self {
        TransactorConfigs {
            private_key: "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".to_string(),
            provider: "http://0.0.0.0:8545".to_string(),
            subgraph: "http://0.0.0.0:8000/subgraphs/id/Qmd5QMJm1zAyfVWGY4bqeFuXogd5gB38uuL2WM51zez1Bd".to_string()
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Transactor {
    configs: TransactorConfigs,
    client: Arc<ContractClient>,
    // contract_addresses: ContractAddresses,
    // TODO: refactor these; only initiate if called by the tx manager
    borrower_ops: BorrowerOperations<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    // escrow_contract: Escrow<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    // token_contract: L2GraphToken<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
}

impl Transactor {
    // Constructor to create a new instance
    pub async fn new() -> Result<Self, anyhow::Error> {
        tracing::debug!("Initialize transaction manager");
        let configs = TransactorConfigs::dummy();
        let provider = Provider::<Http>::try_from(&configs.provider)?;
        let wallet = build_wallet(&configs.private_key)
            .with_chain_id(provider.get_chainid().await.unwrap().as_u64());
        let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

        // Access contracts for the specified chain_id and collateral
        // Later utilize the file that contains the addrs "contracts/deployment-context-latest.json"
        let address_str = "0x9ea962c0efd4792d26e1f3a40e9ed2d4f9681371";
        let borrower_operations_addr: H160 = H160::from_str(address_str).expect("Invalid address");
        let borrower_ops = BorrowerOperations::new(borrower_operations_addr, Arc::new(client.clone()));

        Ok(Transactor {
            client,
            borrower_ops,
            configs,
        })
    }
}

pub fn contract_error_decode(e: String) -> String {
    let encoded_error = &e.to_string()[2..];
    let error_message_hex = &encoded_error[8 + 64..];
    let bytes = hex::decode(error_message_hex).unwrap();
    let message = String::from_utf8(bytes).unwrap();
    tracing::error!(message);
    message
}


/// Build Wallet from Private key or Mnemonic
pub fn build_wallet(value: &str) -> Wallet<SigningKey> {
    value
        .parse::<LocalWallet>().expect("Build wallet from private key")
}
