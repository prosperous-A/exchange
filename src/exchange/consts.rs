pub const OKEX_WS_PUBLIC: &str = "wss://ws.okx.com:8443/ws/v5/public";
pub const OKEX_WS_PRIVATE: &str = "wss://ws.okx.com:8443/ws/v5/private";
pub const OKEX_REST_URL: &str = "https://www.okx.com";

pub const GATE_FUTURES_REST_URL: &str = "https://apiv4-private.gateapi.io";
pub const GATE_FUTURES_COLO_REST_URL: &str = "https://apiv4-private.gateapi.io";
pub const GATE_FUTURES_WS_URL: &str = "wss://fxws-private.gateapi.io/v4/ws/usdt";
pub const GATE_FUTURES_COLO_WS_URL: &str = "wss://fxws-private.gateapi.io/v4/ws/usdt";
pub const BINANCE_FUTURES_REST_URL: &str = "https://fapi.binance.com";
pub const BINANCE_FUTURES_COLO_REST_URL: &str = "https://fapi.binance.com";
pub const BINANCE_FUTURES_WS_URL: &str = "wss://fstream.binance.com/ws";
pub const BINANCE_FUTURES_COLO_WS_URL: &str = "wss://fstream-f.binance.com/ws";

// pub const KUCOIN_FUTURES_REST_URL: &str = "https://api.binance.com";
// pub const KUCOIN_FUTURES_WS_URL: &str = "wss://stream.binance.com:9443/ws";

// pub const GATE_FUTURES_REST_URL: &str = "https://api.gateio.ws";
// pub const GATE_FUTURES_COLO_REST_URL: &str = "https://apiv4-private.gateapi.io";
// pub const GATE_FUTURES_WS_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/usdt";
// pub const GATE_FUTURES_COLO_WS_URL: &str = "wss://fxws-private.gateapi.io/v4/ws/usdt";

// pub const BINANCE_FUTURES_REST_URL: &str = "https://fapi.binance.com";
// pub const BINANCE_FUTURES_COLO_REST_URL: &str = "https://fapi.binance.com";
// pub const BINANCE_FUTURES_WS_URL: &str = "wss://fstream.binance.com/ws";
// pub const BINANCE_FUTURES_COLO_WS_URL: &str = "wss://fstream.binance.com/ws";

pub const BYBIT_FUTURES_REST_URL: &str = "https://api.bybit.com";
pub const BYBIT_FUTURES_PUBLIC_WS_URL: &str = "wss://stream.bybit.com/realtime_public";
pub const BYBIT_FUTURES_PRIVATE_WS_URL: &str = "wss://stream.bybit.com/realtime_private";

pub const BINANCE_SPOT_REST_URL: &str = "https://api.binance.com";
pub const BINANCE_SPOT_WS_URL: &str = "wss://stream.binance.com:9443/ws";

pub const COINEX_FUTURES_REST_URL: &str = "https://api.coinex.com";
pub const COINEX_FUTURES_WS_URL: &str = "wss://socket.coinex.com/v2/futures";

// 如果api.hbdm.com无法访问，可以使用api.btcgateway.pro来做调试，AWS服务器用户推荐使用api.hbdm.vn；
pub const HUOBI_FUTURES_REST_URL: &str = "blackcatapi.hbdm.com";
// pub const HUOBI_FUTURES_PRIVATE_WS_URL: &str = "api.hbdm.com";
// pub const HUOBI_FUTURES_PUBLIC_WS_URL: &str = "api.hbdm.com";

pub const KUCOIN_FUTURES_REST_URL: &str = "https://api-futures.kucoin.com";
pub const KUCOIN_FUTURES_WS_URL: &str = "wss://ws-api.kucoin.com/endpoint";

pub const BITGET_FUTURES_REST_URL: &str = "https://api.bitget.com";
pub const BITGET_FUTURES_WS_PUBLIC_URL: &str = "wss://ws.bitget.com/v2/ws/public";
pub const BITGET_FUTURES_WS_PRIVATE_URL: &str = "wss://ws.bitget.com/v2/ws/private";

pub const BITGET_FUTURES_COLO_REST_URL: &str = "https://vip-api.bitget.com";
pub const BITGET_FUTURES_COLO_WS_PUBLIC_URL: &str = "wss://vip-ws.bitget.com/v2/ws/public";
pub const BITGET_FUTURES_COLO_WS_PRIVATE_URL: &str = "wss://vip-ws.bitget.com/v2/ws/private";

pub const LONG: &str = "long";
pub const SHORT: &str = "short";
pub const COVER_LONG: &str = "cover_long";
pub const COVER_SHORT: &str = "cover_short";

pub const OPEN_SENT: &str = "open_send";
pub const PENDING: &str = "pending";
pub const FILLED: &str = "filled";
pub const CANCELED: &str = "canceled";
pub const ERRORED: &str = "errored";
pub const AMENDING: &str = "amending";

pub const AMEND_OKX: &str = "ws_edit";
pub const AMEND_GATE: &str = "rest_edit";
pub const AMEND_COINEX: &str = "rest_edit_not_client_id";
pub const AMEND_KUCOIN: &str = "rest_edit_not_client_id1";
pub const AMEND_BINANCE: &str = "cancel_retrade";

pub const AMEND_WS_AMEND: &str = "ws_edit";
pub const AMEND_AMEND: &str = "rest_edit";
pub const AMEND_BETCH: &str = "rest_edit_not_client_id";
pub const AMEND_NOT_CLIENT_ID: &str = "rest_edit_not_client_id1";
pub const AMEND_CANCEL_RE_TRADE: &str = "cancel_retrade";

pub const RECONNECT_INTERVAL: u64 = 5;
// static let RECONNECT_INTERVAL = tokio::time::Duration::from_secs(5);

pub const BUY: &str = "buy";
pub const SELL: &str = "sell";

pub const PENDING_INT: u8 = 1;
pub const FILLED_INT: u8 = 2;
pub const CANCELED_INT: u8 = 3;
pub const ERRORED_INT: u8 = 4;
pub const AMENDING_INT: u8 = 5;
pub const OPEN_SENT_INT: u8 = 6;

pub const LONG_INT: u8 = 1;
pub const SHORT_INT: u8 = 2;
pub const COVER_LONG_INT: u8 = 3;
pub const COVER_SHORT_INT: u8 = 4;

pub const TRADES_INT: u8 = 1;
pub const CANCELS_INT: u8 = 2;
pub const AMENDS_INT: u8 = 3;

// pub const ERRORED_INT: u128 = 4;
// pub const AMENDING_INT: u128 = 5;

pub const IP_ORDERLY_COLO1: &str = "172.31.46.61";
pub const IP_ORDERLY_COLO2: &str = "172.31.43.208";
pub const IP_BN_COLO1: &str = "172.31.36.47";
pub fn IP_BITGET_COLO1() -> Vec<String> {
    vec![
        "172.31.37.100".to_string(),
        "172.31.32.120".to_string(),
        "172.31.44.183".to_string(),
        "172.31.45.159".to_string(),
        // "172.31.44.145".to_string(),
    ]
}
