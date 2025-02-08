extern crate mylib; 
use log::info;
use mylib::{
    exchange::{factory, interface::send_request, types::ExchangeConfig},
    tools::logger::init_logger,
};

use ethers::{signers::LocalWallet, types::H160};
use hyperliquid_rust_sdk::{
    BaseUrl, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient, InfoClient,
};

#[tokio::main]
async fn main() { 
    //fixme: aaaa 
    // okx().await;
    hyper().await; 
}


async fn okx() {
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


async fn hyper() { 
    let user: H160 = H160::from_str("0xd067e346e87Cb3cFDeb87f284e5451bE0970D403").unwrap();
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();

    let spot_balance = info_client.user_token_balances(user).await.unwrap();
    info!("spot balance: {:?}", spot_balance);

    let features_balance = info_client.user_state(user).await.unwrap();
    info!("features balance: {:?}", features_balance);

    let fills = info_client.user_fills(user).await.unwrap();
    info!("fills: {:?}", fills);
 
    test_hype_limit_order().await;
 
}

async fn test_hype_limit_order() {
    let wallet: LocalWallet = "0xd067e346e87Cb3cFDeb87f284e5451bE0970D403".parse().unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Mainnet), None, None)
        .await
        .unwrap();

    let order = ClientOrderRequest {
        asset: "ETH".to_string(), // 资产
        is_buy: true,             // true 为做多, false 为做空
        reduce_only: false,       // 只减仓
        limit_px: 1800.0,         // 限价单
        sz: 0.01,                 // 开仓数量
        cloid: None,              // 客户端订单ID, 可选 可手动指定
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }), // 订单类型, Limit 表示限价单, Gtc 表示一直有效直到取消
    };

    // 下单
    let response = exchange_client.order(order, None).await.unwrap();
    info!("Order placed: {response:?}");

    let response = match response {
        ExchangeResponseStatus::Ok(exchange_response) => exchange_response,
        ExchangeResponseStatus::Err(e) => panic!("error with exchange response: {e}"),
    };
    // 获取订单ID
    let status = response.data.unwrap().statuses[0].clone();
    let oid = match status {
        ExchangeDataStatus::Filled(order) => order.oid,
        ExchangeDataStatus::Resting(order) => order.oid,
        _ => panic!("Error: {status:?}"),
    };

    info!("Order ID: {oid}");
}