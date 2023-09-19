pub static ETHERSCAN_API_KEY: &str = "Q88V48Q1GAJ4PIJHWY7RVD38AY1J1KIN3G";

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";

pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";

const COMMENT_DEPTH: i64 = 2;

const get_transaction_by_hash_url: &str = "https://api.etherscan.io/api?module=proxy&action=eth_getTransactionByHash&txhash={}&apikey=YourApiKeyToken";
