use std::net::IpAddr;

use crate::exchange;
use crate::exchange::consts::*;
use crate::exchange::interface::*;
use crate::exchange::types::*;
use crate::tools::util;
use crate::tools::util::get_local_ip_with;
use tracing::error;
use tracing::info;

pub fn new_ws_exg(name: String, mut config: ExchangeConfig) -> Box<dyn WSAPI + Send> {
    if name == "okx_usdt_swap" || name == "okex_usdt_swap" {
        return Box::new(exchange::okx_usdt_swap_ws::Client { config });
    } else {
        error!("not fun exchange: {}", name);
        std::process::exit(1);
    }
}

pub fn new_rest_exg(name: String, mut config: ExchangeConfig) -> Box<dyn RestAPI + Send> {
    if name == "okx_usdt_swap" || name == "okex_usdt_swap" {
        return Box::new(exchange::okx_usdt_swap_rest::Client {
            config,
            client: new_client_http2(),
        });
    } else {
        error!("not fun exchange: {}", name);
        std::process::exit(1);
    }
}

pub fn new_client(config: ExchangeConfig) -> reqwest::Client {
    let ips = util::get_local_ips();
    let ac_index: usize = config.account.parse().unwrap_or(0);
    let index = ac_index % ips.len();
    let ip = ips[index];
    info!("ip: index:{} acc:{} total:{}", index, ac_index, ips.len());
    reqwest::Client::builder()
        .use_rustls_tls()
        .http2_prior_knowledge()
        .local_address(ip)
        .timeout(tokio::time::Duration::from_millis(2000))
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .unwrap()
}

pub fn new_client_http2() -> reqwest::Client {
    reqwest::Client::builder()
        .use_rustls_tls()
        .local_address(util::random_ip())
        .http2_prior_knowledge()
        .timeout(tokio::time::Duration::from_millis(2000))
        .build()
        .unwrap()
}

pub fn new_client_http1() -> reqwest::Client {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(tokio::time::Duration::from_millis(5000))
        .build()
        .unwrap()
}

pub fn new_clients_http2() -> Vec<reqwest::Client> {
    let clients: Vec<reqwest::Client> = (0..util::get_local_ips().len())
        .map(|i| {
            reqwest::Client::builder()
                .use_rustls_tls()
                .http2_prior_knowledge()
                .local_address(util::default_ip(i))
                .timeout(tokio::time::Duration::from_millis(3000))
                .build()
                .unwrap()
        })
        .collect();
    info!("clients: {}", clients.len());
    clients
}

pub fn new_clients_single_ip() -> Vec<reqwest::Client> {
    let length = util::get_local_ips().len();
    if true {
        //length == 1 {
        let clients: Vec<reqwest::Client> = (0..10)
            .map(|i| {
                reqwest::Client::builder()
                    .timeout(tokio::time::Duration::from_millis(2000))
                    .build()
                    .unwrap()
            })
            .collect();
        clients
    } else {
        let clients: Vec<reqwest::Client> = (0..length)
            .map(|i| {
                reqwest::Client::builder()
                    .local_address(util::default_ip(i))
                    .timeout(tokio::time::Duration::from_millis(2000))
                    .build()
                    .unwrap()
            })
            .collect();
        info!("clients: {}", clients.len());
        clients
    }
}

pub fn new_clients_http1() -> Vec<reqwest::Client> {
    let length = util::get_local_ips().len();
    if length <= 0 {
        return vec![reqwest::Client::builder()
            .timeout(tokio::time::Duration::from_millis(2000))
            .build()
            .unwrap()];
    }
    let clients: Vec<reqwest::Client> = (0..length)
        .map(|i| {
            info!("find: {}", util::default_ip(i));
            reqwest::Client::builder()
                .local_address(util::default_ip(i))
                .timeout(tokio::time::Duration::from_millis(2000))
                .build()
                .unwrap()
        })
        .collect();
    info!("clients: {}", clients.len());
    clients
}

pub fn new_clients_with_local_ip(ips: Vec<&str>) -> Vec<reqwest::Client> {
    let clients: Vec<reqwest::Client> = ips
        .iter()
        .map(|i| {
            reqwest::Client::builder()
                .local_address(get_local_ip_with(i))
                .timeout(tokio::time::Duration::from_millis(2000))
                .build()
                .unwrap()
        })
        .collect();
    info!("clients: {}", clients.len());
    clients
}

pub fn new_clients_with_local_ipaddr(ips: Vec<IpAddr>) -> Vec<reqwest::Client> {
    let clients: Vec<reqwest::Client> = ips
        .iter()
        .map(|i| {
            reqwest::Client::builder()
                .local_address(i.clone())
                .timeout(tokio::time::Duration::from_millis(2000))
                .build()
                .unwrap()
        })
        .collect();
    info!("clients: {}", clients.len());
    clients
}
