extern crate mylib; 
use log::info;
use mylib::{
    exchange::{factory, interface::send_request, types::ExchangeConfig},
    tools::logger::init_logger,
};

use ethers::{signers::LocalWallet, types::H160};
use hyperliquid_rust_sdk::{
    BaseUrl, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient, InfoClient, ExchangeResponseStatus, ExchangeDataStatus,ClientCancelRequestCloid,ClientCancelRequest
};
use tungstenite::client; 
use std::str::FromStr;
use std::{thread::sleep, time::Duration};
use uuid::Uuid;


async fn hyper() { 
  let _g = init_logger();
  let user: H160 = H160::from_str("0xd067e346e87Cb3cFDeb87f284e5451bE0970D403").unwrap();
  let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();

  // let spot_balance = info_client.user_token_balances(user).await.unwrap();
  // info!("spot balance: {:?}", spot_balance);

  // let features_balance = info_client.user_state(user).await.unwrap();
  // info!("features balance: {:?}", features_balance);

  // let fills = info_client.user_fills(user).await.unwrap();
  // info!("fills: {:?}", fills);

  // let open_orders = info_client.open_orders(user).await.unwrap();
  // info!("open orders: {:?}", open_orders);

  // let user_state = info_client.user_state(user).await.unwrap();
  // info!("user state: {:?}", user_state);

  // let user_token_balances = info_client.user_token_balances(user).await.unwrap();
  // info!("user token balances: {:?}", user_token_balances);

  // let meta_info = info_client.meta().await.unwrap();
  // info!("Metadata: {:?}", meta_info);


  // test_hype_limit_order().await; 
  // order_and_cancel_cloid().await;
  order_and_cancel().await;
}



async fn test_hype_limit_order() {
  let wallet: LocalWallet = "0x3959f4558de32d59e0271165dfb8782a732702701d7e3c79bdd56133ad4953c6"
  .parse()
  .unwrap();

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

async fn order_and_cancel_cloid() {
  let wallet: LocalWallet = "3959f4558de32d59e0271165dfb8782a732702701d7e3c79bdd56133ad4953c6"
      .parse()
      .unwrap();

  let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Mainnet), None, None)
      .await
      .unwrap();

  // Order and Cancel with cloid
  let cloid = Uuid::new_v4();
  let order = ClientOrderRequest {
      asset: "ETH".to_string(),
      is_buy: true,
      reduce_only: false,
      limit_px: 1800.0,
      sz: 0.01,
      cloid: Some(cloid),
      order_type: ClientOrder::Limit(ClientLimit {
          tif: "Gtc".to_string(),
      }),
  };

  let response = exchange_client.order(order, None).await.unwrap();
  info!("Order placed: {response:?}");

  // So you can see the order before it's cancelled
  sleep(Duration::from_secs(10));

  let cancel = ClientCancelRequestCloid {
      asset: "ETH".to_string(),
      cloid,
  };

  // This response will return an error if order was filled (since you can't cancel a filled order), otherwise it will cancel the order
  let response = exchange_client.cancel_by_cloid(cancel, None).await.unwrap();
  info!("Order potentially cancelled: {response:?}");
}


async fn order_and_cancel() {
let wallet: LocalWallet = "3959f4558de32d59e0271165dfb8782a732702701d7e3c79bdd56133ad4953c6"
      .parse()
      .unwrap();

  let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Mainnet), None, None)
      .await
      .unwrap();

  let order = ClientOrderRequest {
      asset: "ETH".to_string(),
      is_buy: true,
      reduce_only: false,
      limit_px: 1800.0,
      sz: 0.01,
      cloid: None,
      order_type: ClientOrder::Limit(ClientLimit {
          tif: "Gtc".to_string(),
      }),
  };

  let response = exchange_client.order(order, None).await.unwrap();
  info!("Order placed: {response:?}");

  let response = match response {
      ExchangeResponseStatus::Ok(exchange_response) => exchange_response,
      ExchangeResponseStatus::Err(e) => panic!("error with exchange response: {e}"),
  };
  let status = response.data.unwrap().statuses[0].clone();
  let oid = match status {
      ExchangeDataStatus::Filled(order) => order.oid,
      ExchangeDataStatus::Resting(order) => order.oid,
      _ => panic!("Error: {status:?}"),
  };

  // So you can see the order before it's cancelled
  sleep(Duration::from_secs(10));

  let cancel = ClientCancelRequest {
      asset: "ETH".to_string(),
      oid,
  };

  // This response will return an error if order was filled (since you can't cancel a filled order), otherwise it will cancel the order
  let response = exchange_client.cancel(cancel, None).await.unwrap();
  info!("Order potentially cancelled: {response:?}");
}