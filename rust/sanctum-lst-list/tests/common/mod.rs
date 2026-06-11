use lazy_static::lazy_static;
use reqwest::Client;
use sanctum_lst_list::{SanctumLst, SanctumLstList};

lazy_static! {
    pub static ref SANCTUM_LST_LIST: SanctumLstList = SanctumLstList::load();
}

pub const SOLANA_RPC_URL: &str =
    "https://mainnet.helius-rpc.com/?api-key=0a0cddc9-942e-43d0-90ef-6cf3f4475d92";
const TEST_USER_AGENT: &str = "glamsystems/sanctum-lst-list-tests";

pub fn http_client() -> Client {
    Client::builder()
        .user_agent(TEST_USER_AGENT)
        .build()
        .unwrap()
}

pub fn find_sanctum_lst_by_symbol_unwrapped(symbol: &str) -> &SanctumLst {
    SANCTUM_LST_LIST
        .sanctum_lst_list
        .iter()
        .find(|lst| lst.symbol == symbol)
        .unwrap()
}
