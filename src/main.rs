use pyth_sdk_solana::{load_price_feed_from_account, Price};
use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;


fn main() {
    //crypto.LUNA/USD
    let url = "http://api.mainnet-beta.solana.com";
    let key = "5bmWuR1dgP4avtGYMNKLuxumZTVKGgoN2BCMXWDNL9nY";
    let clnt = RpcClient::new(url.to_string());
    let price_key = Pubkey::from_str(key).unwrap();

    let mut price_account = clnt.get_account(&price_key).unwrap();
    //let price_account_info: AccountInfo = "5bmWuR1dgP4avtGYMNKLuxumZTVKGgoN2BCMXWDNL9nY";
    let price_feed =
            load_price_feed_from_account(&price_key, &mut price_account).unwrap();
    let current_price: Price = price_feed.get_current_price().unwrap();
    println!("price: ({} +- {}) x 10^{}", current_price.price, current_price.conf, current_price.expo);
}
