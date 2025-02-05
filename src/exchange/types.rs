use super::consts::*;
use crate::tools::util::{get_avg_with_weight, round, timestamp13};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ExchangeName {
    Binance,
    Okx,
    Bybit,
    Gate,
    Huobi,
    Coinex,
    OkxSpot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsChannel {
    Ticker,
    Depth,
    Deal,
    Depth100MS,
    Trades,
    Order,
    OkxSendOrder,
}

impl WsChannel {
    pub fn v(&self) -> String {
        match *self {
            WsChannel::Ticker => "ticker".to_string(),
            WsChannel::Depth100MS => "depth100ms".to_string(),
            WsChannel::Trades => "trades".to_string(),
            WsChannel::Order => "order".to_string(),
            WsChannel::Deal => "Deal".to_string(),
            WsChannel::Depth => "depth".to_string(),
            WsChannel::OkxSendOrder => "OkxSendOrder".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum StdWsCallback {
    Ticker(StdTicker),
    // TickerRc(std::rc::Rc<StdTicker>),
    Depth(StdDepth),
    Trade(StdTrade),
    Orders(Vec<StdOrder>),
    Order(StdOrder),
    Test(tokio::time::Instant),
}

#[derive(Debug, Clone)]
pub struct InitItem {
    pub market: Market,
    pub long: f64,
    pub short: f64,
    pub rights: f64,
    pub min_pos: f64,
    pub max_pos: f64,
    pub each_amount: f64,
    pub config: ExchangeConfig,
}

#[derive(Debug, Clone)]
pub struct StdOrder {
    pub symbol: String,
    pub price: f64,
    pub amount: f64,
    pub fill_amount: f64,
    pub avg_price: f64,
    pub client_id: String,
    pub order_id: String,
    pub side: String,
    pub deal_side: String,
    pub pnl: f64,
    pub fee: f64,
    pub order_type: String,
    pub status: String,
    pub is_maker: bool,

    pub ws_event_time: u128,
    pub ws_match_time: u128,
    // pub ws_cost: u128,
    pub recv_time: tokio::time::Instant,
    pub rest_trade_cost: u128,
    pub rest_amend_cost: u128,
    pub shm_index: u128,
}

impl StdOrder {
    pub fn new() -> Self {
        StdOrder {
            price: 0.0,
            amount: 0.0,
            fill_amount: 0.0,
            avg_price: 0.0,
            fee: 0.0,
            client_id: "".to_string(),
            order_id: "".to_string(),
            side: "".to_string(),
            deal_side: "".to_string(),
            pnl: 0.0,
            order_type: "".to_string(),
            status: "".to_string(),
            symbol: "".to_string(),
            ws_event_time: 0,
            ws_match_time: 0,
            recv_time: tokio::time::Instant::now(),
            rest_trade_cost: 0,
            rest_amend_cost: 0,
            is_maker: false,
            shm_index: 0,
            // ws_cost: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StdTicker {
    // pub new_bids: [[f64; 2]; DEPTH_LIMIT],
    pub symbol: String,
    pub bid_price: f64,
    pub bid_amount: f64,
    pub ask_price: f64,
    pub ask_amount: f64,
    pub ws_event_time: u128,
    pub recv_time: tokio::time::Instant,
    pub recv_ts: u128,
    pub parse_cost: u128,
    pub trans_cost: u128,
    pub update_id: u128,
}

impl StdTicker {
    pub fn new() -> Self {
        StdTicker {
            symbol: String::new(),
            bid_price: 0.0,
            bid_amount: 0.0,
            ask_amount: 0.0,
            ask_price: 0.0,
            ws_event_time: 0,
            recv_time: tokio::time::Instant::now(),
            parse_cost: 0,
            trans_cost: 0,
            update_id: 0,
            recv_ts: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteTicker {
    pub bid_price: f64,
    pub ask_price: f64,
    pub ask_amount: f64,
    pub bid_amount: f64,
    pub ts: u64,
    pub update_id: u64,
}

impl RemoteTicker {
    pub fn new() -> Self {
        RemoteTicker {
            bid_price: 0.0,
            bid_amount: 0.0,
            ask_amount: 0.0,
            ask_price: 0.0,
            ts: 0,
            update_id: 0,
        }
    }
}

const DEPTH_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct StdDepth {
    // pub new_bids: [[f64; 2]; DEPTH_LIMIT],
    pub bids: Vec<Vec<f64>>,
    pub asks: Vec<Vec<f64>>,
    pub ws_event_time: u128,
    pub ws_match_time: u128,
    pub recv_time: tokio::time::Instant,
    pub parse_cost: u128,
    pub trans_cost: u128,
    pub update_id: u128,
    pub recv_ts: u128,
}

#[derive(Debug, Clone)]
pub struct StdTrade {
    pub price: f64,
    pub amount: f64,
    pub side: String,
    pub recv_time: tokio::time::Instant,
    pub recv_ts: u128,
    // pub ws_match_time: i64,
}
impl StdTrade {
    pub fn new() -> Self {
        StdTrade {
            price: 0.0,
            amount: 0.0,
            side: "".to_string(),
            recv_time: tokio::time::Instant::now(),
            recv_ts: 0,
        }
    }
}

impl StdDepth {
    pub fn new() -> Self {
        StdDepth {
            bids: vec![],
            asks: vec![],
            ws_event_time: 0,
            ws_match_time: 0,
            recv_time: tokio::time::Instant::now(),
            parse_cost: 0,
            trans_cost: 0,
            update_id: 0,
            recv_ts: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExchangeConfig {
    pub trader_name: String,
    pub pricer_name: String,
    pub symbol: String,
    pub account: String,
    pub coin: String,
    pub base: String,
    pub access_key: String,
    pub secret_key: String,
    pub password: String,

    pub params: Value,
    pub lever: f64,
    pub max_lever: f64,
    pub opens: Vec<f64>,
    pub closes: Vec<f64>,
    pub avoids: Vec<f64>,
    pub avoids_price: Vec<f64>,
    pub exchange: String,
    pub ref_exchange: String,

    pub colo: bool,
    pub proxy: String,
    pub ip: usize,
    pub wsuri: String,
}

#[derive(Clone, Debug)]
pub struct LeacyItem {
    pub data: Vec<u128>,
    pub index: usize,
    pub avg: u128,
    pub cur: u128,
    pub full: bool,
    pub size: usize,
}

static LEACY_LIMIT: usize = 500;

impl LeacyItem {
    pub fn new(size: Option<usize>) -> LeacyItem {
        LeacyItem {
            size: size.unwrap_or(LEACY_LIMIT),
            data: vec![0; size.unwrap_or(LEACY_LIMIT)],
            index: 0,
            avg: 0,
            full: false,
            cur: 0,
        }
    }

    pub fn get_avg(&mut self) -> u128 {
        let sum: u128 = self.data.iter().sum();
        let mut len = self.index;
        if self.full {
            len = self.size;
        }
        if self.index == 0 {
            return 0;
        }
        let avg: u128 = sum / (len.max(1) as u128);
        self.avg = avg;
        avg
    }

    pub fn to_stirng(&mut self) -> String {
        let avg = self.get_avg();
        let min: u128 = self.data.iter().min().unwrap_or(&0).clone();
        let max: u128 = self.data.iter().max().unwrap_or(&0).clone();
        return format!("min:{} max:{} cur:{} avg:{}", min, max, self.cur, avg);
    }

    pub fn update(&mut self, value: u128) {
        self.index += 1;
        if self.index >= self.size {
            self.index = 1;
            self.full = true;
        }
        self.data[self.index] = value;
        self.cur = value;
    }
}

// #[derive(Clone, Debug)]
// pub struct LeacyItem {
//     pub begin: u128,
//     pub end: u128,
//     pub cost: u128,
//     pub avg: f64,
// }

// impl LeacyItem {
//     pub fn new() -> LeacyItem {
//         LeacyItem {
//             begin: 0,
//             end: 0,
//             cost: 0,
//             avg: 0.0,
//         }
//     }
//     pub fn reset(&mut self) {
//         self.begin = timestamp13();
//     }
//     pub fn update(&mut self) {
//         self.end = timestamp13();
//         self.cost = self.end - self.begin;
//         self.avg = round(get_avg_with_weight(self.avg, self.cost as f64, 0.999), 1);
//     }
//     pub fn update_with(&mut self, begin: u128, end: u128) {
//         self.cost = end - begin;
//         self.avg = round(get_avg_with_weight(self.avg, self.cost as f64, 0.999), 1);
//     }
// }

#[derive(Debug, Clone)]
pub struct SHMOrder {
    pub action: u128,
    pub index: u128,
    pub recv: tokio::time::Instant,
    pub order: GridOrder,
    // pub hello: String,
    // pub test: Test1,
    // pub orders: Vec<&'a Order>,
}

// #[derive(Debug, Clone)]
// pub struct Test1 {
//     pub num: u128,
// }

impl SHMOrder {
    pub fn new() -> Self {
        SHMOrder {
            action: 0,
            // orders: vec![],
            index: 0,
            recv: tokio::time::Instant::now(),
            // test: Test1 { num: 1 },
            order: GridOrder::new("", 0.0, 0.0, 0),
            // hello: "231".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RestLeacy {
    pub market: LeacyItem,
    pub account: LeacyItem,
    pub position: LeacyItem,
    pub positions: LeacyItem,
    pub cancel_all: LeacyItem,
    pub cancel: LeacyItem,
    pub trade: LeacyItem,
    pub amend: LeacyItem,
}

impl RestLeacy {
    pub fn new() -> RestLeacy {
        RestLeacy {
            market: LeacyItem::new(None),
            account: LeacyItem::new(None),
            position: LeacyItem::new(None),
            positions: LeacyItem::new(None),
            cancel_all: LeacyItem::new(None),
            cancel: LeacyItem::new(None),
            trade: LeacyItem::new(None),
            amend: LeacyItem::new(None),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub symbol: String,
    pub order_id: String,
    pub client_id: String,
    pub update_ts: u128,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct OrderList {
    pub orders: Vec<Order>,
    pub raw: Value,
}

impl ExchangeConfig {
    pub fn new(symbol: &str, access_key: &str, secret_key: &str, password: &str) -> ExchangeConfig {
        ExchangeConfig {
            trader_name: "".to_string(),
            pricer_name: "".to_string(),
            account: "0".to_string(),
            coin: "".to_string(),
            base: "".to_string(),
            symbol: symbol.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            password: password.to_string(),
            params: Value::Null,
            lever: 1.0,
            max_lever: 1.0,
            exchange: "".to_string(),
            ref_exchange: "".to_string(),
            opens: vec![0.001],
            closes: vec![0.0001],
            avoids: vec![0.00005],
            avoids_price: vec![0.00005],
            colo: false,
            proxy: "".to_string(),
            ip: 12345,
            wsuri: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridOrder {
    pub price: f64,
    pub amount: f64,
    pub status: String,
    pub shift: usize,
    pub side: String,
    pub client_id: String,
    pub create_at: u128,
    pub update_at: u128,
    pub pnl: f64,
    pub amend_count: usize,
    pub order_type: String,
    pub symbol: String,
    pub order_id: String,
    pub index: u128,
}

impl GridOrder {
    pub fn new(side: &str, price: f64, amount: f64, shift: usize) -> GridOrder {
        GridOrder {
            price,
            amount,
            status: OPEN_SENT.to_string(),
            shift,
            side: side.to_string(),
            client_id: "".to_string(),
            create_at: timestamp13(),
            update_at: timestamp13(),
            pnl: 0.0,
            amend_count: 0,
            order_type: "limit".to_string(),
            symbol: "".to_string(),
            order_id: "".to_string(),
            index: 0,
            // client_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridOrder1 {
    pub price: f64,
    pub amount: f64,
    pub status: u8,
    pub shift: usize,
    pub side: u8,
    pub client_id: u128,
    pub create_at: u128,
    pub update_at: u128,
    pub pnl: f64,
    pub amend_count: usize,
    pub index: u128,
    pub order_type: u8,
    pub order_id: u128,
    pub action: u8,
    pub account: u128,
}

impl GridOrder1 {
    pub fn new(side: u8, price: f64, amount: f64, shift: usize) -> GridOrder1 {
        GridOrder1 {
            price,
            amount,
            status: OPEN_SENT_INT,
            shift,
            side: side,
            client_id: 0,
            create_at: timestamp13(),
            update_at: timestamp13(),
            pnl: 0.0,
            amend_count: 0,
            order_type: 0,
            order_id: 0,
            index: 0,
            action: TRADES_INT,
            account: 0,
            // client_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Market {
    pub contract_val: f64,
    pub price_decimal: i32,
    pub amount_decimal: i32,
    pub min_amount: f64,
    pub tick_size: f64,
    pub raw: Value,
}

impl Market {
    pub fn new(raw: Value) -> Market {
        Market {
            contract_val: 1.0,
            price_decimal: 16,
            amount_decimal: 16,
            min_amount: 0.000001,
            tick_size: 0.000001,
            raw: Value::Null,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Asset {
    pub coin: String,
    pub free: f64,
    pub total: f64,
    pub locked: f64,
    pub usdval: f64,
}

#[derive(Clone, Debug)]
pub struct Account {
    pub usdt_rights: f64,
    pub asset: Vec<Asset>,
    pub raw: Value,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub long: f64,
    pub short: f64,
    pub net: f64,
    pub raw: Value,
}

impl Position {
    pub fn new(net: f64, raw: Value) -> Position {
        Position {
            long: 0.0,
            short: 0.0,
            net: net,
            raw,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PositionList {
    pub positions: HashMap<String, Position>,
    pub raw: Value,
}

#[derive(Clone, Debug)]
pub struct MyRequest {
    pub uri: String,
    pub method: reqwest::Method,
    pub body: Value,
    pub form: Value,
    pub params: Value,
    pub headers: reqwest::header::HeaderMap,
}

#[derive(Clone, Debug)]
pub struct MyRequest1 {
    pub uri: String,
    pub method: reqwest::Method,
    pub body: Value,
    // pub t: T,
    pub form: Value,
    pub params: Value,
    pub headers: reqwest::header::HeaderMap,
}
