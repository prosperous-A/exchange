// use super::okex_rest::Order;
use crate::exchange::consts::*;
use crate::exchange::interface::*;
use crate::exchange::types::*;
use crate::tools::util;
use crate::tools::util::get_local_ips;
use crate::tools::util::get_local_ips_with_eni_idx;
use crate::tools::util::timestamp13;
// use crate::tools::util::timestamp19;
// use async_http_proxy::http_connect_tokio;
use futures::SinkExt;
use futures::StreamExt;
use rand::prelude::SliceRandom;
use serde_json::json;
use serde_json::Value;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio::time::Duration;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::*;
use tokio_tungstenite::{
    client_async_tls, connect_async,
    tungstenite::{Message, Result},
};
use tracing::{error, info};
use url::Url;

#[derive(Clone)]
pub struct WebSocketClientFeed {
    pub channel: WsChannel,
    pub config: ExchangeConfig,
    pub subscribes_handler: fn(WsChannel, ExchangeConfig) -> (String, Vec<Value>),
    pub ticker_handler: fn(&str, &mut StdTicker) -> Option<StdTicker>,
    pub trade_handler: fn(&str, &mut StdTrade) -> Option<StdTrade>,
    pub depth_handler: fn(&str, &mut StdDepth) -> Option<StdDepth>,
    pub order_handler: fn(&str, &mut StdOrder) -> Option<Vec<StdOrder>>,
    // pub message_handler: fn(&str, &WsChannel) -> Option<StdWsCallback>,
}

impl WebSocketClientFeed {
    async fn connect_ws(
        &mut self,
        wsuri: String,
    ) -> Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>> {
        let uri = Url::parse(&wsuri).unwrap();
        let config = self.config.clone();
        if let Ok(result) = tokio::time::timeout(
            Duration::from_secs(3),
            connect_stream(self.channel.clone(), uri, config),
        )
        .await
        {
            if let Ok(ws_stream) = result {
                return Some(ws_stream);
            }
        }
        return None;
    }

    pub async fn get_connect(&mut self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let (wsuri, subscribes) =
            (self.subscribes_handler)(self.channel.clone(), self.config.clone());
        info!("connected to : {:?}", wsuri);
        self.config.wsuri = wsuri.clone();
        let mut ws_stream;
        loop {
            match self.connect_ws(wsuri.clone()).await {
                Some(ws) => {
                    ws_stream = ws;
                    break;
                }
                None => {
                    info!("connect failed... wait");
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
        for sub in subscribes.iter() {
            info!("subscribe {}", sub);
            let subscribe_message = Message::Text(sub.to_string());
            if let Err(error) = ws_stream.send(subscribe_message).await {
                error!("failed to send subscribe request: {}", error);
            }
            if subscribes.len() > 1 {
                tokio::time::sleep(Duration::from_millis(500)).await;
                if wsuri.contains("bitget") {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
        return ws_stream;
    }

    pub async fn get_connect_withuri(
        &mut self,
        _wsuri: String,
    ) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let (wsuri, subscribes) =
            (self.subscribes_handler)(self.channel.clone(), self.config.clone());

        info!("connected to : {:?}", _wsuri);
        self.config.wsuri = _wsuri.clone();
        let mut ws_stream;
        loop {
            match self.connect_ws(_wsuri.clone()).await {
                Some(ws) => {
                    ws_stream = ws;
                    break;
                }
                None => {
                    info!("connect failed");
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
        for sub in subscribes.iter() {
            info!("subscribe {}", sub);
            let subscribe_message = Message::Text(sub.to_string());
            if let Err(error) = ws_stream.send(subscribe_message).await {
                error!("failed to send subscribe request: {}", error);
            }
            if subscribes.len() > 1 {
                tokio::time::sleep(Duration::from_millis(500)).await;
                if _wsuri.contains("bitget") || _wsuri.contains("coinex") {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
        return ws_stream;
    }

    pub fn get_ping(&mut self) -> Message {
        let (wsuri, _) = (self.subscribes_handler)(self.channel.clone(), self.config.clone());
        let mut message = Message::Pong(vec![]);
        if wsuri.contains("binance") {
            message = Message::Pong(vec![]);
        } else if wsuri.contains("okx") {
            message = Message::Text("ping".to_string());
        } else {
            error!("try to send ping not fun type {}", wsuri);
        }
        message
    }
}

async fn connect_stream(
    channel: WsChannel,
    mut case_url: Url,
    config: ExchangeConfig,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let (client, _) = connect_async(case_url).await?;
    return Ok(client);
}
