use ethers::contract::{abigen, Contract};
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet};
use ethers_core::{
    k256::ecdsa::SigningKey,
    types::{Bytes, TransactionReceipt, H160, U256},
    utils::keccak256,
};


use hex::decode;
use byteorder::{BigEndian, ByteOrder};


use std::collections::HashMap;
use std::str::FromStr;

use crate::transactor::contract_error_decode;

use super::Transactor;

pub type NetworkContracts =
    HashMap<String, Contract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>;
pub type ContractAddresses = HashMap<String, H160>;
pub type ContractClient = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

abigen!(
    BorrowerOperations,
    "../contracts/out/BorrowerOperations.sol/BorrowerOperations.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

impl Transactor {
    /// Grab batch manager current settings
    pub async fn get_interest_batch_manager(&self, address: &str) -> Result<InterestBatchManager, anyhow::Error> {
        let address: H160 = H160::from_str(address).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let value = self
            .borrower_ops
            .get_interest_batch_manager(address)
            .call()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        tracing::debug!(manager = tracing::field::debug(&value), "Get interest batch manager"); 

        Ok(value)
    }

    // set Batch Manager Annual Interest Rate; 
    // using constant param 0 for upper hint, 0 for lower hint, 0 for max upfront fee
    pub async fn set_annual_interest_rate(
        &self,
        new_rate: u128,
    ) -> Result<(), anyhow::Error> {
        let populated_tx = self.borrower_ops.set_batch_manager_annual_interest_rate(
            new_rate, U256::zero(), U256::zero(), U256::zero(),
        );
        let estimated_gas = populated_tx
            .estimate_gas()
            .await
            .map_err(|e| anyhow::anyhow!(contract_error_decode(e.to_string())))?;
        tracing::debug!(
            estimated_gas = tracing::field::debug(&estimated_gas),
            "estimate gas"
        );

        // Attempt to send the populated tx with estimated gas, can later add a slippage
        let tx_result = populated_tx
            .gas(estimated_gas)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(contract_error_decode(e.to_string())))?
            .await
            .map_err(|e| anyhow::anyhow!(contract_error_decode(e.to_string())))?;
        tracing::debug!(
            value = tracing::field::debug(&tx_result),
            "set batch manager annual interest rate call result"
        );
        Ok(())
    }
}
