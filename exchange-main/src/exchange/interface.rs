use crate::exchange::consts::*;
use crate::exchange::types::*;
use crate::exchange::websocket::WebSocketClientFeed;
use async_trait::async_trait;
use dyn_clone::DynClone;
use serde_json::json;
use serde_json::Value;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Result;
use tracing::{error, info};

pub trait WSAPI {
    fn new(&mut self, channels: Vec<WsChannel>) -> WebSocketClientFeed;
}

#[async_trait::async_trait]
pub trait RestAPI: dyn_clone::DynClone {
    fn get_client(&mut self) -> &reqwest::Client;
    fn symbol(&mut self) -> String;
    fn amend_type(&mut self) -> String;
    fn format_amount(&mut self, amount: f64, market: Market) -> f64;
    fn format_price(&mut self, price: f64, market: &Market) -> f64;
    fn uuid(&mut self) -> String;
    fn get_leacy(&mut self) -> String;

    fn get_market(&mut self, symbol: String) -> MyRequest;
    fn parse_market(&mut self, symbol: String, res: &Value) -> Result<Market>;
    fn get_account_config(&mut self) -> MyRequest;
    fn get_account(&mut self) -> MyRequest;
    fn parse_account(&mut self, res: &Value) -> Result<Account>;

    fn get_positions(&mut self) -> MyRequest;

    fn parse_positions(&mut self, res: &Value) -> Result<PositionList>;
    fn cancel_all(&mut self, symbol: String) -> MyRequest;
    fn cancel_order(&mut self, symbol: String, order_id: String, client_id: String) -> MyRequest;

    fn batch_create_order(&mut self, orders: Vec<GridOrder>, params: Value) -> MyRequest;
    async fn create_order(
        &mut self,
        symbol: String,
        side: String,
        price: f64,
        size: f64,
        order_type: String,
        client_oid: String,
        params: Value,
    ) -> MyRequest;

    fn parse_order(&mut self, res: &Value) -> Result<StdOrder>;

    fn batch_amend_order(&mut self, orders: Vec<GridOrder>, params: Value) -> MyRequest;
    async fn amend_order(
        &mut self,
        symbol: String,
        new_price: f64,
        new_amount: f64,
        order_id: String,
        client_oid: String,
        params: Value,
    ) -> MyRequest;

    fn set_lever(&mut self, symbol: String, side: String, lever: u8) -> MyRequest;

    async fn liqu_close(&mut self, symbol: String, p: Position) -> MyRequest;

    fn sign(
        &mut self,
        method: reqwest::Method,
        uri: &str,
        params: Value,
        data: Value,
        auth: bool,
    ) -> MyRequest;
}

pub async fn send_request(client: &reqwest::Client, req: MyRequest) -> Result<Value> {
    if req.method == reqwest::Method::TRACE {
        return Ok(Value::Null);
    }
    // info!("req uri {:?} {}", req.uri, req.method.to_string());
    // info!("req form {:?}", req.form);
    // info!("req body {:?}", req.body);
    // info!("req params {:?}", req.params);
    let reqq = req.clone();
    let mut request = client.request(req.method, req.uri).headers(req.headers);
    if req.form.is_null() == false {
        request = request.form(&req.form);
    }
    if req.body.is_null() == false {
        request = request.json(&req.body);
        // request = request.body(req.body.to_string());
    }
    if req.params.is_null() == false {
        request = request.query(&req.params);
    }

    let resp = request.send().await;
    // info!("resp {:?}", resp);
    match resp {
        Ok(res) => {
            let status = res.status().as_u16();
            if status >= 200 && status <= 206 {
                // info!("resp: {}", res.json().await.unwrap());
                return Ok(res.json().await.unwrap_or(json!({})));
            } else {
                let res_headers = res.headers().clone();
                let text = res.text().await.unwrap_or("parse error".to_string());
                if text.contains("NO_CHANGE") {
                    info!("request:  {} {}", status, text);
                } else {
                    info!("req form {}", reqq.form.to_string());
                    info!("req body {}", reqq.body.to_string());
                    info!("req params {}", reqq.params.to_string());
                    info!("req headers {:?}", reqq.headers);
                    info!("resp headers: {:?}", res_headers);
                    error!("request: {} {}", reqq.uri, reqq.method);
                    error!("request:  {} {}", status, text);
                }
                return Ok(Value::Null);
            }
        }
        Err(err) => {
            info!("req form {}", reqq.form.to_string());
            info!("req body {}", reqq.body.to_string());
            info!("req params {}", reqq.params.to_string());
            info!("req headers {:?}", reqq.headers);
            error!("request: {} {} {}", err, reqq.uri, reqq.method.to_string());
            Ok(Value::Null)
        }
    }
    // Ok(Value::Null)
}

pub fn send_request_sync(client: &reqwest::blocking::Client, req: MyRequest) -> Result<Value> {
    if req.method == reqwest::Method::TRACE {
        return Ok(Value::Null);
    }
    // info!("req uri {:?} {}", req.uri, req.method.to_string());
    // info!("req form {:?}", req.form);
    // info!("req body {:?}", req.body);
    // info!("req params {:?}", req.params);
    let reqq = req.clone();
    let mut request = client.request(req.method, req.uri).headers(req.headers);
    if req.form.is_null() == false {
        request = request.form(&req.form);
    }
    if req.body.is_null() == false {
        request = request.json(&req.body);
        // request = request.body(req.body.to_string());
    }
    if req.params.is_null() == false {
        request = request.query(&req.params);
    }

    let resp = request.send();
    // info!("resp {:?}", resp);
    match resp {
        Ok(res) => {
            let status = res.status().as_u16();
            if status >= 200 && status <= 206 {
                // info!("resp: {}", res.json().unwrap());
                return Ok(res.json().unwrap_or(json!({})));
            } else {
                let text = res.text().unwrap_or("parse error".to_string());
                if text.contains("NO_CHANGE") {
                    info!("request:  {} {}", status, text);
                } else {
                    info!("req form {:?}", reqq.form.to_string());
                    info!("req body {:?}", reqq.body.to_string());
                    info!("req params {:?}", reqq.params.to_string());
                    info!("req headers {:?}", reqq.headers);
                    error!("request: {} {}", reqq.uri, reqq.method.to_string());
                    error!("request:  {} {}", status, text);
                }
                return Ok(Value::Null);
            }
        }
        Err(err) => {
            info!("req form {:?}", reqq.form.to_string());
            info!("req body {:?}", reqq.body.to_string());
            info!("req params {:?}", reqq.params.to_string());
            info!("req headers {:?}", reqq.headers);
            error!("request: {} {} {}", err, reqq.uri, reqq.method.to_string());
            Ok(Value::Null)
        }
    }
    // Ok(Value::Null)
}

pub fn send_request_sync_(client: &reqwest::blocking::Client, req: MyRequest1) -> Result<Value> {
    if req.method == reqwest::Method::TRACE {
        return Ok(Value::Null);
    }
    // info!("req uri {:?} {}", req.uri, req.method.to_string());
    // info!("req form {:?}", req.form);
    // info!("req body {:?}", req.body);
    // info!("req params {:?}", req.params);
    let reqq = req.clone();
    let mut request = client.request(req.method, req.uri).headers(req.headers);
    if req.form.is_null() == false {
        request = request.form(&req.form);
    }
    if req.body.is_null() == false {
        request = request.json(&req.body);
        // request = request.body(req.body.to_string());
    }
    if req.params.is_null() == false {
        request = request.query(&req.params);
    }
    // if req.t.is_empty() == false {
    //     request = request.json(&req.t);
    // }

    let resp = request.send();
    // info!("resp {:?}", resp);
    match resp {
        Ok(res) => {
            let status = res.status().as_u16();
            if status >= 200 && status <= 206 {
                // info!("resp: {}", res.json().unwrap());
                return Ok(res.json().unwrap_or(json!({})));
            } else {
                let text = res.text().unwrap_or("parse error".to_string());
                if text.contains("NO_CHANGE") {
                    info!("request:  {} {}", status, text);
                } else {
                    info!("req form {:?}", reqq.form.to_string());
                    info!("req body {:?}", reqq.body.to_string());
                    info!("req params {:?}", reqq.params.to_string());
                    info!("req headers {:?}", reqq.headers);
                    error!("request: {} {}", reqq.uri, reqq.method.to_string());
                    error!("request:  {} {}", status, text);
                }
                return Ok(Value::Null);
            }
        }
        Err(err) => {
            info!("req form {:?}", reqq.form.to_string());
            info!("req body {:?}", reqq.body.to_string());
            info!("req params {:?}", reqq.params.to_string());
            info!("req headers {:?}", reqq.headers);
            error!("request: {} {} {}", err, reqq.uri, reqq.method.to_string());
            Ok(Value::Null)
        }
    }
    // Ok(Value::Null)
}
