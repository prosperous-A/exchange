use super::consts::*;
use super::types::*;
use crate::exchange::interface::send_request;
use crate::exchange::interface::RestAPI;
use crate::tools::util::*;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_json::json;
use serde_json::{Value, Value::Null};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Result;
use tracing::error;
use tracing::info;

#[derive(Clone)]
pub struct Client {
    pub config: ExchangeConfig,
    pub client: reqwest::Client,
}

pub const OKX_POS_MODE: &str = "net"; //"isolated";
pub const OKX_TD_MODE: &str = "cross"; //"isolated";

#[async_trait::async_trait]
impl RestAPI for Client {
    fn get_client(&mut self) -> &reqwest::Client {
        &self.client
    }
    fn amend_type(&mut self) -> String {
        AMEND_OKX.to_string()
    }
    fn get_leacy(&mut self) -> String {
        "".to_string()
    }
    fn format_price(&mut self, price: f64, market: &Market) -> f64 {
        // round(
        //     (price / market.tick_size) as i64 as f64 * market.tick_size,
        //     market.price_decimal,
        // )
        round_down(price, market.price_decimal)
    }
    fn format_amount(&mut self, amount: f64, market: Market) -> f64 {
        (amount as i64 as f64).max(1.0)
    }
    fn symbol(&mut self) -> String {
        return self.config.symbol.to_string().replace("-SWAP", "") + "-SWAP";
    }
    fn uuid(&mut self) -> String {
        return okex_uuid();
    }
    fn get_market(&mut self, mut symbol: String) -> MyRequest {
        let p = json!({
            "instType": "SWAP",
        });
        self.sign(Method::GET, "/api/v5/public/instruments", p, Null, false)
    }
    fn parse_market(&mut self, symbol: String, res: &Value) -> Result<Market> {
        let mut result = Market::new(res.clone());
        for item in res["data"].as_array().unwrap_or(&vec![]).iter() {
            if symbol == item["instId"] {
                let tick = item["tickSz"].as_str().unwrap().parse::<f64>().unwrap();
                result.contract_val = item["ctVal"].as_str().unwrap().parse::<f64>().unwrap();
                result.price_decimal = (tick.log10() as i32) * -1;
                result.amount_decimal = 0;
                result.tick_size = tick;
                result.min_amount = item["minSz"].as_str().unwrap().parse::<f64>().unwrap();
                result.raw = Value::Null;
                break;
            }
        }
        Ok(result)
    }

    fn get_account(&mut self) -> MyRequest {
        self.sign(Method::GET, "/api/v5/account/balance", Null, Null, true)
    }
    fn parse_account(&mut self, res: &Value) -> Result<Account> {
        let mut rights = 0.0;
        for item in res["data"].as_array().unwrap_or(&vec![]).iter() {
            let amount = item["totalEq"].as_str().unwrap().parse::<f64>().unwrap();
            rights += amount;
        }
        Ok(Account {
            usdt_rights: rights,
            asset: vec![],
            raw: res.clone(),
        })
    }

    fn get_positions(&mut self) -> MyRequest {
        self.sign(Method::GET, "/api/v5/account/positions", Null, Null, true)
    }

    fn parse_positions(&mut self, res: &Value) -> Result<PositionList> {
        let mut positions: HashMap<String, Position> = HashMap::new();
        for item in res["data"].as_array().unwrap_or(&vec![]).iter() {
            let symbol = item["instId"].as_str().unwrap();
            let side = item["posSide"].as_str().unwrap();
            let amount = item["pos"].as_str().unwrap().parse::<f64>().unwrap();
            let mgnMode = item["mgnMode"].as_str().unwrap();
            let posSide = item["posSide"].as_str().unwrap();
            if symbol == self.symbol() && posSide == OKX_POS_MODE && mgnMode == OKX_TD_MODE {
            } else {
                let p = json!({
                    "instId": symbol,
                    "posSide": posSide,
                    "mgnMode": mgnMode,
                });
                info!(
                    "liqu {} {} {} {} {}",
                    symbol, side, amount, mgnMode, posSide
                );
                let req = self.sign(Method::POST, "/api/v5/trade/close-position", Null, p, true);
                let client = self.client.clone();
                tokio::task::spawn(async move {
                    let resp = send_request(&client, req).await;
                    info!("liqu pos{:?}", resp);
                });
                continue;
            }
            let mut p: Position = Position {
                long: 0.0,
                short: 0.0,
                net: 0.0,
                raw: item.clone(),
            };
            if let Some(cur) = positions.get(symbol) {
                p.long = cur.long;
                p.short = cur.short;
            }
            if side == "long" {
                p.long = amount;
            } else if side == "short" {
                p.short = amount.abs();
            } else if side == "net" {
                if amount > 0.0 {
                    p.long = amount;
                } else if amount < 0.0 {
                    p.short = amount.abs();
                }
            }
            positions.insert(symbol.to_string(), p);
        }
        Ok(PositionList {
            positions,
            raw: Null,
        })
    }

    fn cancel_all(&mut self, symbol: String) -> MyRequest {
        info!("cancel_all");
        let client = self.client.clone();
        let config = self.config.clone();
        tokio::task::spawn(async move {
            let req = local_sign(
                Method::GET,
                "/api/v5/trade/orders-pending",
                Null,
                Null,
                true,
                &config.clone(),
            );
            let res = send_request(&client, req).await;
            let data = parse_open_orders(&res.unwrap()).unwrap();
            let orders: Vec<Value> = data
                .orders
                .iter()
                .map(|o| {
                    json!({
                        "instId":o.symbol.clone(),
                        "ordId": o.order_id.clone(),
                    })
                })
                .take(19)
                .collect();
            if orders.len() > 0 {
                info!("cancel_all {:?}", orders.len());
                let req = local_sign(
                    Method::POST,
                    "/api/v5/trade/cancel-batch-orders",
                    Null,
                    json!(orders),
                    true,
                    &config.clone(),
                );
                let res = send_request(&client, req).await;
                info!("cancel_all {:?}", res);
            }
        });
        self.sign(Method::TRACE, "", Null, Null, false)
    }

    async fn create_order(
        &mut self,
        symbol: String,
        side: String,
        price: f64,
        size: f64,
        order_type: String,
        client_oid: String,
        params: Value,
    ) -> MyRequest {
        let body = json!({
            "instId": symbol,
            "tdMode": OKX_TD_MODE,
            "clOrdId": client_oid,
            "side": side.to_lowercase(),
            "ordType": order_type.to_lowercase(),
            "sz": size,
            "px": price,
        });
        self.sign(Method::POST, "/api/v5/trade/order", Null, body, true)
    }
    fn parse_order(&mut self, res: &Value) -> Result<StdOrder> {
        Ok(StdOrder::new())
    }
    fn cancel_order(&mut self, symbol: String, order_id: String, client_id: String) -> MyRequest {
        self.sign(Method::HEAD, "", Null, Null, true)
    }

    async fn amend_order(
        &mut self,
        symbol: String,
        new_price: f64,
        new_amount: f64,
        order_id: String,
        client_oid: String,
        params: Value,
    ) -> MyRequest {
        self.sign(Method::TRACE, "", Null, Null, false)
    }

    fn batch_amend_order(&mut self, orders: Vec<GridOrder>, params: Value) -> MyRequest {
        self.sign(Method::TRACE, "", Null, Null, false)
    }
    fn batch_create_order(&mut self, orders: Vec<GridOrder>, params: Value) -> MyRequest {
        self.sign(Method::TRACE, "", Null, Null, false)
    }
    fn set_lever(&mut self, symbol: String, side: String, lever: u8) -> MyRequest {
        let client = self.client.clone();
        let p = json!({
            "instId": symbol,
            "lever": format!("{}", lever),
            "mgnMode": OKX_TD_MODE,
            // "posSide": side,
        });
        let req = self.sign(Method::POST, "/api/v5/account/set-leverage", Null, p, true);
        tokio::task::spawn(async move {
            let res = send_request(&client, req).await;
            info!("set_lever: {:?}", res);
        });
        let client = self.client.clone();
        let p = json!({
            "posMode": "net_mode"
        });
        let req = self.sign(
            Method::POST,
            "/api/v5/account/set-position-mode",
            Null,
            p,
            true,
        );
        tokio::task::spawn(async move {
            let res = send_request(&client, req).await;
            info!("set_pos_mode: {:?}", res);
        });
        self.sign(Method::TRACE, "", Null, Null, false)
    }

    async fn liqu_close(&mut self, symbol: String, p: Position) -> MyRequest {
        let params = json!({
            "instId": symbol,
            "mgnMode": OKX_TD_MODE,
        });
        self.sign(
            Method::POST,
            "/api/v5/trade/close-position",
            Null,
            params,
            true,
        )
    }
    fn get_account_config(&mut self) -> MyRequest {
        self.sign(Method::TRACE, "", Null, Null, false)
    }

    fn sign(
        &mut self,
        method: reqwest::Method,
        uri: &str,
        params: Value,
        data: Value,
        auth: bool,
    ) -> MyRequest {
        local_sign(method, uri, params, data, auth, &self.config)
    }
}

fn local_sign(
    method: reqwest::Method,
    uri: &str,
    params: Value,
    data: Value,
    auth: bool,
    config: &ExchangeConfig,
) -> MyRequest {
    let ts = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    if auth {
        let mut body_str = "".to_string();
        if method == "GET" {
        } else {
            body_str = data.to_string();
        }
        let build_str = format!("{}{}{}{}", ts, method, uri, body_str);
        let sign = build_okex_sign(&config.secret_key, &build_str);
        headers.insert("OK-ACCESS-KEY", config.access_key.parse().unwrap());
        headers.insert("OK-ACCESS-SIGN", sign.parse().unwrap());
        headers.insert("OK-ACCESS-PASSPHRASE", config.password.parse().unwrap());
        headers.insert("OK-ACCESS-TIMESTAMP", format!("{}", ts).parse().unwrap());
    }
    let mut req = MyRequest {
        method: method.clone(),
        uri: "".to_string(),
        body: Null,
        form: Null,
        params: Null,
        headers,
    };
    let mut url_str = format!("{}{}", OKEX_REST_URL, uri);
    if method == reqwest::Method::GET {
        if params != Value::Null {
            url_str = url_str + "?" + &params_to_str(params);
        }
    }
    if method == reqwest::Method::POST {
        req.body = data;
    }
    req.uri = url_str;
    req
}

fn parse_open_orders(res: &Value) -> Result<OrderList> {
    let mut orders = vec![];
    for detail in res["data"].as_array().unwrap_or(&vec![]).iter() {
        let order_id = detail["ordId"].as_str().unwrap().to_string();
        let client_id = detail["clOrdId"].as_str().unwrap().to_string();
        let symbol = detail["instId"].as_str().unwrap().to_string();
        let update_ts = detail["uTime"]
            .as_str()
            .unwrap()
            .to_string()
            .parse::<u128>()
            .unwrap();
        orders.push(Order {
            symbol,
            order_id,
            client_id,
            update_ts,
            status: PENDING.to_string(),
        });
    }
    Ok(OrderList { orders, raw: Null })
}
