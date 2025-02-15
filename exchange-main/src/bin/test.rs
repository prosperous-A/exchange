extern crate mylib;
use futures::StreamExt;
use log::info;
use mylib::{
    exchange::{
        consts::LONG,
        factory::{self, new_ws_exg},
        interface::send_request,
        types::{ExchangeConfig, GridOrder, StdOrder, StdTicker, WsChannel},
    },
    tools::{logger::init_logger, util::gate_uuid},
};
use serde_yaml::Value;

#[tokio::main]
async fn main() {
    let _g = init_logger();
    let symbol = "BTC-USDT";
    let exchange = "okx_usdt_swap";
    let account = "00";
    let access_key = "";
    let secret_key = "";
    let password = "";
    let mut config = ExchangeConfig::new(symbol, access_key, secret_key, password);
    config.account = account.to_string();
    config.exchange = exchange.to_string();

    let mut rest = factory::new_rest_exg(exchange.to_string().clone(), config.clone());

    // 交易对配置
    let request = rest.get_market(symbol.to_string());
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);
    let market = rest.parse_market(symbol.to_string(), &response.unwrap());
    info!("res: {:?}", market.unwrap());

    // 取消订单
    let request = rest.cancel_all(symbol.to_string());
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    // 取消订单
    let request = rest.cancel_order(symbol.to_string(), "".to_string(), "".to_string());
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    //get account
    let request = rest.get_account();
    let client = rest.get_client();
    let response = send_request(client, request).await;
    let account = rest.parse_account(&response.unwrap());
    info!("response: {:?}", account);

    //get position
    let request = rest.get_positions();
    let client = rest.get_client();
    let response = send_request(client, request).await;
    let positions = rest.parse_positions(&response.unwrap());
    info!("response: {:?}", positions);

    //trade
    let request = rest
        .create_order(
            symbol.to_string(),
            LONG.to_string(),
            90000.0,
            0.01,
            "limit".to_string(),
            gate_uuid(),
            serde_json::Value::Null,
        )
        .await;
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    //batch trade
    let order = GridOrder::new(LONG, 90000.0, 0.01, 0);
    let request = rest.batch_create_order(vec![order], serde_json::Value::Null);
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    //amend
    let request = rest
        .amend_order(
            symbol.to_string(),
            91000.0,
            0.01,
            "".to_string(),
            "".to_string(),
            serde_json::Value::Null,
        )
        .await;
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    // batch amend
    let mut order = GridOrder::new(LONG, 90000.0, 0.01, 0);
    order.client_id = "".to_string();
    let request = rest.batch_amend_order(vec![order], serde_json::Value::Null);
    let client = rest.get_client();
    let response = send_request(client, request).await;
    info!("response: {:?}", response);

    let mut ws_client = new_ws_exg(config.exchange.clone(), config.clone());
    let mut ticker_ws_client = ws_client.new(vec![WsChannel::Ticker]);
    let mut ticker_ws = ticker_ws_client.get_connect().await;

    let mut ws_client = new_ws_exg(config.exchange.clone(), config.clone());
    let mut order_ws_client = ws_client.new(vec![WsChannel::Order]);
    let mut order_ws = order_ws_client.get_connect().await;

    let mut std_ticker = StdTicker::new();
    let mut std_order = StdOrder::new();

    let mut is_init = false;
    let orders: Vec<GridOrder> = vec![];

    let mut inerval = tokio::time::interval(tokio::time::Duration::from_secs(60));
    inerval.tick().await;

    loop {
        tokio::select! {
            _ = inerval.tick() => {
                is_init = false;
            }
            Some(Ok(msg)) = order_ws.next() =>{
                if let Some(data) = (order_ws_client.order_handler)(&msg.to_string(), &mut std_order) {
                    // receive websocket order, update local orders
                    info!("recv order :{:?}", data);
                }
            }
             Some(Ok(msg)) = ticker_ws.next() =>{
                if let Some(data) = (ticker_ws_client.ticker_handler)(&msg.to_string(), &mut std_ticker) {
                    info!("recv: {}", data.bid_price);
                    if is_init == false {
                        // init env begin

                        // get market
                        // cancel orders
                        // get account
                        // get position
                        // liqu other positions
                        //
                        // send orders
                        is_init =true;

                    } else {
                        // check orders
                        // if ticker changes,
                        // cancel order / amend order / send order
                    }
                }
            }
        }
        // if let Some(Ok(msg)) = ws.next().await {
        //     if let Some(data) = (ws_client1.ticker_handler)(&msg.to_string(), &mut std_ticker) {
        //         info!("recv: {}", data.bid_price);
        //         if is_init == false {
        //             // init env begin

        //             // get market
        //             // cancel orders
        //             // get account
        //             // get position
        //             // liqu other positions
        //             //
        //             // send orders
        //             is_init =true;

        //         } else {
        //             // check orders
        //             // if ticker changes,
        //             // cancel order / amend order / send order
        //         }
        //     }
        // }
    }
}
