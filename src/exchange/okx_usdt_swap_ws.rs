use crate::exchange::consts::*;
use crate::exchange::interface::WSAPI;
use crate::exchange::types::*;
use crate::exchange::websocket::WebSocketClientFeed;
use crate::tools::util::*;
use async_trait::async_trait;
use fast_float;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tracing::{error, info, warn};
#[derive(Deserialize)]
pub struct TickerData {
    data: Vec<Ticker>,
}

#[derive(Deserialize)]
pub struct Ticker {
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
    ts: String,
    // action: String,
    // checksum: i32,
}

#[derive(Deserialize, Debug)]
pub struct EmptyOrderData {
    data: Vec<EmptyOrder>,
}

#[derive(Deserialize, Debug)]
pub struct EmptyOrder {
    ordId: String,
    clOrdId: String,
    sCode: String,
    sMsg: String,
}

#[derive(Deserialize, Debug)]
pub struct FullOrderData {
    data: Vec<FullOrder>,
}

#[derive(Deserialize, Debug)]
pub struct FullOrder {
    pnl: String,
    ordId: String,
    clOrdId: String,
    state: String,
    accFillSz: String,
    uTime: String,
    avgPx: String,
}

#[derive(Deserialize)]
pub struct BBoData {
    pub data: Vec<BBo>,
}
#[derive(Deserialize)]
pub struct BBo {
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
    ts: String,
}

#[derive(Deserialize)]
pub struct TradeData {
    data: Vec<Trade>,
}
#[derive(Deserialize)]
pub struct Trade {
    px: String,
    sz: String,
    side: String,
}

#[derive(Deserialize)]
pub struct DepthData {
    data: Vec<Depth>,
    action: String,
}

#[derive(Deserialize)]
pub struct Depth {
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
    ts: String,
    checksum: i32,
}

pub struct Client {
    pub config: ExchangeConfig,
}

impl WSAPI for Client {
    fn new(&mut self, channels: Vec<WsChannel>) -> WebSocketClientFeed {
        WebSocketClientFeed {
            config: self.config.clone(),
            channel: channels[0].clone(),
            subscribes_handler,
            ticker_handler,
            order_handler,
            depth_handler,
            trade_handler,
        }
    }
}

pub fn subscribes_handler(channel: WsChannel, mut config: ExchangeConfig) -> (String, Vec<Value>) {
    let mut subscribes: Vec<Value> = vec![];
    let mut url = OKEX_WS_PUBLIC.to_string().clone();
    let mut symbol = config.symbol.replace("-SWAP", "").replace("1000", "") + "-SWAP";
    config.symbol = symbol.clone();
    if symbol.contains("-USDT") == false {
        symbol = symbol.replace("USDT", "-USDT");
    }
    if symbol.contains("_") == true {
        symbol = symbol.replace("_", "");
    }
    if channel == WsChannel::Ticker {
        subscribes.push(json!({
            "op": "subscribe",
            "args": json!([{
                "channel": "bbo-tbt",
                "instId": symbol.clone(),
            }]),
        }));
    }
    let login = get_login_str(
        Some(config.access_key.clone()),
        Some(config.secret_key.clone()),
        Some(config.password.clone()),
    );
    if channel == WsChannel::Depth {
        // subscribes.push(login.clone());
        // subscribes.push(json!({
        //     "op": "subscribe",
        //     "args": json!([{
        //         "channel": "books50-l2-tbt",
        //         "instId": symbol,
        //     }]),
        // }));
        subscribes.push(json!({
            "op": "subscribe",
            "args": json!([{
                "channel": "bbo-tbt",
                "instId": symbol.clone(),
            }]),
        }));
    }
    if channel == WsChannel::Depth100MS {
        // subscribes.push(login.clone());
        // subscribes.push(json!({
        //     "op": "subscribe",
        //     "args": json!([{
        //         "channel": "books5",
        //         "instId": symbol,
        //     }]),
        // }));
    }
    if channel == WsChannel::Trades {
        subscribes.push(json!({
            "op": "subscribe",
            "args": json!([{
                "channel": "trades",
                "instId": symbol,
            }]),
        }));
    }
    if channel == WsChannel::Order {
        subscribes.push(login.clone());
        subscribes.push(json!({
            "op": "subscribe",
            "args": json!([{
                "channel": "orders",
                "instType": "SWAP",
                "instId": symbol,
            }]),
        }));
        url = OKEX_WS_PRIVATE.to_string().clone();
    }
    if channel == WsChannel::OkxSendOrder {
        subscribes.push(login.clone());
        subscribes.push(json!({
            "op": "subscribe",
            "args": json!([{
                "channel": "positions",
                "instType": "SWAP",
                "instId": symbol,
            }]),
        }));
        url = OKEX_WS_PRIVATE.to_string().clone();
    }
    (url, subscribes)
}
pub fn depth_handler(msg: &str, ticker: &mut StdDepth) -> Option<StdDepth> {
    None
}
pub fn trade_handler(msg: &str, trade: &mut StdTrade) -> Option<StdTrade> {
    None
}
pub fn ticker_handler(msg: &str, ticker: &mut StdTicker) -> Option<StdTicker> {
    let begin = tokio::time::Instant::now();
    let data = match serde_json::from_str::<BBoData>(&msg) {
        Ok(message) => message,
        Err(e) => {
            if msg.contains("ping") || msg.contains("pong") || msg.contains("login") {
                info!("recv ping: {}", msg);
            } else {
                warn!("ticker_handler message: {} {}", msg, e)
            }
            return None;
        }
    };
    ticker.parse_cost = begin.elapsed().as_nanos();

    if data.data[0].bids.len() > 0 {
        ticker.bid_price = fast_float::parse(&data.data[0].bids[0][0]).unwrap();
        ticker.bid_amount = fast_float::parse(&data.data[0].bids[0][1]).unwrap();
    }
    if data.data[0].asks.len() > 0 {
        ticker.ask_price = fast_float::parse(&data.data[0].asks[0][0]).unwrap();
        ticker.ask_amount = fast_float::parse(&data.data[0].asks[0][1]).unwrap();
    }
    ticker.ws_event_time = data.data[0].ts.parse().unwrap_or(0);
    ticker.recv_time = begin;
    ticker.update_id = ticker.ws_event_time;
    ticker.trans_cost = begin.elapsed().as_nanos();
    Some(ticker.clone())
}

pub fn order_handler(msg: &str, order: &mut StdOrder) -> Option<Vec<StdOrder>> {
    // info!("recv: {}", msg);
    let begin = tokio::time::Instant::now();
    if msg.contains("accFillSz") {
        // fillorder
        let data = match serde_json::from_str::<FullOrderData>(&msg) {
            Ok(message) => message,
            Err(e) => {
                if msg.contains("ping") || msg.contains("pong") || msg.contains("login") {
                } else {
                    warn!("order_handler message: {} {}", msg, e)
                }
                return None;
            }
        };
        if data.data.len() <= 0 {
            return None;
        }
        let orders: Vec<StdOrder> = data
            .data
            .iter()
            .map(|o| {
                let mut status = "";
                if o.state == "live" {
                    status = PENDING;
                } else if o.state == "filled" {
                    status = FILLED;
                    // info!("got {}", msg);
                } else if o.state == "canceled" {
                    status = CANCELED;
                } else if o.state == "partially_filled" {
                    status = PENDING;
                    // info!("part deal {:?}", o);
                } else {
                    error!("error order status {:?}", o);
                }
                order.avg_price = fast_float::parse(&o.avgPx).unwrap_or(0.0);
                order.pnl = fast_float::parse(&o.pnl).unwrap_or(0.0);
                order.client_id = o.clOrdId.to_string();
                order.order_id = o.ordId.to_string();
                order.status = status.to_string();
                order.recv_time = begin;
                order.ws_event_time = o.uTime.parse().unwrap_or(0);

                return order.clone();
            })
            .collect();
        return Some(orders);
    } else {
        let data = match serde_json::from_str::<EmptyOrderData>(&msg) {
            Ok(message) => message,
            Err(_) => {
                if msg.contains("subscribe") {
                    info!("parse failed {}", msg);
                } else if msg.contains("ping") || msg.contains("pong") {
                } else {
                    error!("parse failed EmptyOrderData: {}", msg);
                }
                return None;
            }
        };
        if data.data.len() <= 0 {
            return None;
        }

        let orders: Vec<StdOrder> = data
            .data
            .iter()
            .filter(|o| {
                if (o.sCode == "0") == false {
                    info!("order failed: {:?}", o);
                }
                return o.sCode == "0";
            })
            .map(|o| {
                let status = PENDING.to_string();
                order.client_id = o.clOrdId.to_string();
                order.order_id = o.ordId.to_string();
                order.status = status;
                order.recv_time = begin;
                order.ws_event_time = timestamp13();
                return order.clone();
            })
            .collect();
        return Some(orders);
    }
}

pub fn get_login_str(
    access_key: Option<String>,
    secret_key: Option<String>,
    password: Option<String>,
) -> Value {
    let ts = timestamp();
    let access = access_key.unwrap_or("".to_string());
    let secret = secret_key.unwrap_or("".to_string());
    let pass = password.unwrap_or("".to_string());
    let b64_encoded_sig = build_okex_sign(secret.as_str(), &format!("{}GET/users/self/verify", ts));
    json!({
        "op": "login",
        "args":json!([json!({
            "apiKey": access,
            "passphrase": pass,
            "timestamp": ts,
            "sign" :b64_encoded_sig,
        })])
    })
}
