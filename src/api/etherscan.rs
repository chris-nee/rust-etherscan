use serde::{Deserialize, Serialize};

use crate::constants::ETHERSCAN_API_KEY;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub blockHash: String,
    pub blockNumber: String,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub maxFeePerGas: String,
    pub maxPriorityFeePerGas: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transactionIndex: String,
    pub value: String,
    pub r#type: String,
    pub accessList: Vec<String>,
    pub chainId: String,
    pub v: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionItem {
    pub jsonrpc: String,
    pub id: i128,
    pub result: TransactionDetail,
}

pub async fn get_transaction_by_hash(hash: String) -> Result<TransactionItem, reqwest::Error> {
    let url = format!("https://api.etherscan.io/api?module=proxy&action=eth_getTransactionByHash&txhash={}&apikey={}", hash, ETHERSCAN_API_KEY);
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EtherLastPriceResult {
    pub ethbtc: String,
    pub ethbtc_timestamp: String,
    pub ethusd: String,
    pub ethusd_timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EtherLastPriceResponse {
    pub status: String,
    pub message: String,
    pub result: EtherLastPriceResult,
}

pub async fn get_ether_last_price() -> Result<EtherLastPriceResponse, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=stats&action=ethprice&apikey={}",
        ETHERSCAN_API_KEY
    );
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalEtherSupplyResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

pub async fn get_ether_total_supply() -> Result<TotalEtherSupplyResponse, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=stats&action=ethsupply&apikey={}",
        ETHERSCAN_API_KEY
    );
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalEther2SupplyDetails {
    pub EthSupply: String,
    pub Eth2Staking: String,
    pub BurntFees: String,
    pub WithdrawnTotal: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalEther2SupplyResponse {
    pub status: String,
    pub message: String,
    pub result: TotalEther2SupplyDetails,
}

pub async fn get_ether2_total_supply() -> Result<TotalEther2SupplyResponse, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=stats&action=ethsupply2&apikey={}",
        ETHERSCAN_API_KEY
    );
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalEtherNodeCountDetails {
    pub UTCDate: String,
    pub TotalNodeCount: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalEtherNodeCountResponse {
    pub status: String,
    pub message: String,
    pub result: TotalEtherNodeCountDetails,
}

pub async fn get_ether_node_count() -> Result<TotalEtherNodeCountResponse, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=stats&action=nodecount&apikey={}",
        ETHERSCAN_API_KEY
    );
    reqwest::get(&url).await?.json().await
}
