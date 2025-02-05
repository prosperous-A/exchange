extern crate mylib;
use log::info;
use mylib::{
    exchange::{factory, interface::send_request, types::ExchangeConfig},
    tools::logger::init_logger,
};

#[tokio::main]
async fn main() {
    let _g = init_logger();
    let symbol = "BTC-USDT";
    let exchange = "okx_usdt_swap";
    let account = "00";
    let access_key = "660fb70b-2ed1-4ceb-8724-56312d006216";
    let secret_key = "9EA688B308597E4FA8FCD841F2D2E3DF";
    let password = "Ydb.15838890945";
    let mut config = ExchangeConfig::new(symbol, access_key, secret_key, password);
    config.account = account.to_string();
    config.exchange = exchange.to_string();

    let mut rest = factory::new_rest_exg(exchange.to_string().clone(), config.clone());

    let reqest = rest.get_market(symbol.to_string());
    let client = rest.get_client();
    let response = send_request(client, reqest).await;
    let market = rest.parse_market(symbol.to_string(), &response.unwrap());
    info!("res: {:?}", market.unwrap());
}
