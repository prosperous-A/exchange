use chrono::format::format;
use hmac::{Hmac, Mac};
use pnet_datalink;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use serde_json::{Value, Value::Null};
use sha2::Digest;
use sha2::Sha256;
use sha2::Sha512;
use std::hash::Hash;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
use std::collections::HashMap;

use crate::exchange::types::ExchangeConfig;
pub fn convert_okex_side(side: &str) -> [String; 2] {
    return match side {
        "long" => ["buy".to_string(), "long".to_string()],
        "short" => ["sell".to_string(), "short".to_string()],
        "cover_long" => ["sell".to_string(), "long".to_string()],
        "cover_short" => ["buy".to_string(), "short".to_string()],
        _ => ["".to_string(), "".to_string()],
    };
}

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn timestamp13() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn okex_uuid() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    rand_string
}

pub fn gate_uuid() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(26)
        .map(char::from)
        .collect();
    format!("t-{}", rand_string)
}

pub fn round(number: f64, rounding: i32) -> f64 {
    let scale: f64 = 10_f64.powi(rounding);
    (number * scale).round() / scale
}

pub fn round_up(number: f64, rounding: i32) -> f64 {
    let scale: f64 = 10_f64.powi(rounding);
    (number * scale).ceil() / scale
}

pub fn round_down(number: f64, rounding: i32) -> f64 {
    let scale: f64 = 10_f64.powi(rounding);
    (number * scale).floor() / scale
}

pub fn build_okex_sign(secret: &str, data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    base64::encode(code_bytes)
}

pub fn build_binance_sign(secret: &str, data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    format!("{:X}", code_bytes)
}
pub fn build_bybit_sign(secret: &str, data: &str) -> String {
    // info!("data:{}", data);
    // info!("secret:{}", secret);

    // let mut mac =
    //     HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    // mac.update(data.as_bytes());
    // let result = mac.finalize();
    // let code_bytes = result.into_bytes();
    // return format!("{:x}", code_bytes);

    let key: ring::hmac::Key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_bytes());
    let signature = ring::hmac::sign(&key, data.as_bytes());
    hex::encode(signature.as_ref())

    // // info!("{}", res);
    // // info!("{}", hex_string);
    // hex_string
}

pub fn params_to_str(params: Value) -> String {
    if params.is_null() {
        return "".to_string();
    }
    let arr: Vec<String> = params
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap().to_string()))
        .collect();
    arr.join("&")
}

pub fn get_local_ips() -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = vec![];
    for iface in pnet_datalink::interfaces() {
        for item in iface.ips.clone().into_iter() {
            // && iface.name != "lo"
            if item.is_ipv4() {
                // info!("find ip: {:?}", item);
                ips.push(item.ip());
            }
        }
    }
    ips.remove(0);
    ips
}

pub fn get_local_ips_with_eni_idx(index: u32) -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = vec![];
    for iface in pnet_datalink::interfaces() {
        for item in iface.ips.clone().into_iter() {
            // name: "ens5"
            if item.is_ipv4() && iface.index == index {
                // info!("find ip: {:?}", item);
                ips.push(item.ip());
            }
        }
    }
    // ips.remove(0);
    ips
}

pub fn get_local_ip_with(ip: &str) -> IpAddr {
    let mut ips: IpAddr = IpAddr::from_str("0.0.0.0").unwrap();
    for iface in pnet_datalink::interfaces() {
        for item in iface.ips.clone().into_iter() {
            if item.is_ipv4() && item.ip().to_string().contains(ip) {
                ips = item.ip();
            }
        }
    }
    ips
}

pub fn random_ip() -> IpAddr {
    let ips = get_local_ips();
    let num = thread_rng().gen_range(0..ips.len());
    let cur = ips[num].clone();
    // info!("use ip {:?} //// {}, {}/{}", ips, cur, num, ips.len());
    cur
}

pub fn default_ip(num: usize) -> IpAddr {
    if num == 12345 {
        return random_ip();
    }
    let ips = get_local_ips();
    let cur = ips[num % ips.len()].clone();
    // info!("use ip {:?} //// {}, {}/{}", ips, cur, num, ips.len());
    cur
}

pub fn find_gather_price(items: Vec<Vec<f64>>, amount: f64, cv: f64) -> f64 {
    let mut price = items[0][0];
    let mut total = 0.0;
    for item in items {
        price = item[0];
        total += item[1] * cv;
        if total >= amount {
            break;
        }
    }
    price
}

pub async fn random_sleep() {
    let begin = 1 * 1000;
    let end = 5 * 1000;
    let num = thread_rng().gen_range(begin..end);
    info!("sleep {}", num);
    tokio::time::sleep(std::time::Duration::from_millis(num)).await;
}

pub fn get_avg(old: f64, new: f64) -> f64 {
    // 1000 调数据平滑， 大约是1ms，
    if old == 0.0 {
        new
    } else {
        old * 0.999 + new * (1.0 - 0.999)
    }
}

pub fn get_avg_with_weight(old: f64, new: f64, weight: f64) -> f64 {
    // 1000 调数据平滑， 大约是1ms，
    if old == 0.0 {
        new
    } else {
        old * weight + new * (1.0 - weight)
    }
}

pub fn merge_value_and_map(v: &Value, fields: &HashMap<String, String>) -> Value {
    match v {
        Value::Object(m) => {
            let mut m = m.clone();
            for (k, v) in fields {
                m.insert(k.clone(), Value::String(v.clone()));
            }
            Value::Object(m)
        }
        v => v.clone(),
    }
}

pub fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn map_to_query_string(fields: &HashMap<String, String>) -> String {
    if fields.is_empty() {
        return "".to_string();
    }
    let mut url_str = "".to_string();
    for (i, (k, v)) in fields.iter().enumerate() {
        url_str = format!("{}{}={}&", url_str, k, v);
    }
    url_str[0..url_str.len() - 1].to_string()
}

pub fn value_to_query_string(fields: &Value) -> String {
    if fields.is_null() {
        return "".to_string();
    }
    let mut url_str = "".to_string();
    // info!("value: {:?}", fields);
    for (k, v) in fields.as_object().unwrap() {
        if let Some(v) = v.as_str() {
            url_str = format!("{}{}={}&", url_str, k.as_str(), v);
        } else {
            url_str = format!("{}{}={}&", url_str, k.as_str(), v);
        }
    }
    url_str[0..url_str.len() - 1].to_string()
}

pub fn value_to_query_string_encode(fields: &Value) -> String {
    if fields.is_null() {
        return "".to_string();
    }
    let mut url_str = "".to_string();
    // info!("value: {:?}", fields);
    for (k, v) in fields.as_object().unwrap() {
        if let Some(v) = v.as_str() {
            url_str = format!("{}{}={}&", url_str, k.as_str(), urlencoding::encode(v));
        } else {
            url_str = format!(
                "{}{}={}&",
                url_str,
                k.as_str(),
                urlencoding::encode(&format!("{}", v))
            );
        }
    }
    url_str[0..url_str.len() - 1].to_string()
}

pub fn value_to_query_string_encode_sort(fields: &Value) -> String {
    if fields.is_null() {
        return "".to_string();
    }
    let mut url_str = "?".to_string();
    // info!("value: {:?}", fields);
    let obj = fields.as_object().unwrap();
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort();
    for k in keys {
        let v = obj.get(k).unwrap();
        if let Some(v) = v.as_str() {
            url_str = format!("{}{}={}&", url_str, k.as_str(), urlencoding::encode(v));
        } else {
            url_str = format!(
                "{}{}={}&",
                url_str,
                k.as_str(),
                urlencoding::encode(&format!("{}", v))
            );
        }
    }
    if url_str == "?" {
        return "".to_string();
    } else {
        return url_str[0..url_str.len() - 1].to_string();
    }
}

pub fn value_to_query_string_encode_backpack(fields: &Value) -> String {
    if fields.is_null() {
        return "".to_string();
    }
    let mut url_str = "".to_string();
    // info!("value: {:?}", fields);
    let obj = fields.as_object().unwrap();
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort();
    for k in keys {
        let v = obj.get(k).unwrap();
        if let Some(v) = v.as_str() {
            url_str = format!("{}{}={}&", url_str, k.as_str(), urlencoding::encode(v));
        } else {
            url_str = format!(
                "{}{}={}&",
                url_str,
                k.as_str(),
                urlencoding::encode(&format!("{}", v))
            );
        }
    }
    if url_str == "" {
        return "".to_string();
    } else {
        return url_str[0..url_str.len() - 1].to_string();
    }
}

pub fn hash_string_to_range(s: &str, range: u64) -> i32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    let _s = s
        .replace("bybit_usdt_swap_v6", "bybit_usdt_swap_v5")
        .replace("_colo", "");
    _s.hash(&mut hasher);
    let hash_value = hasher.finish() as u64;
    // 使用取模运算将散列值映射到1到range的范围内
    // 注意：由于我们的范围是从1开始，所以这里使用(hash_value % range) + 1
    let res = ((hash_value % (range - 2000)) + 1) as i32 + 2000;
    info!("{} hash to {}", _s, res);
    res
}

pub fn get_cpu_nums() -> usize {
    let content = std::fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut num_physical_cores = 0;
    for line in content.lines() {
        if line.starts_with("processor") {
            num_physical_cores += 1;
        }
    }
    if num_physical_cores == 0 {
        return core_affinity::get_core_ids().unwrap().len();
    }
    num_physical_cores
}

pub fn check_can_trade(config: ExchangeConfig) -> bool {
    use chrono::Timelike;
    // 不是， 直接可以交易。
    if config.exchange.contains("coinex") == false {
        info!("checkfunding true");
        return true;
    }
    let ts = timestamp();
    // 将10位时间戳转换为DateTime<Utc>
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(ts.try_into().unwrap(), 0).unwrap();

    // 获取小时和分钟
    let hour = dt.hour();
    let minute = dt.minute();

    // 定义funding times (UTC)
    let begin_funding_times = [23, 7, 15];
    let funding_times = [0, 8, 16];

    for &begin_funding_hour in &begin_funding_times {
        // 检查是否在funding time
        if hour == begin_funding_hour && minute > 40 {
            info!("checkfunding false {} {}", hour, minute);
            return false;
        }
    }
    for &funding_hour in &funding_times {
        // 检查是否在funding time
        if hour == funding_hour && minute < 20 {
            info!("checkfunding false {} {}", hour, minute);
            return false;
        }
    }
    info!("checkfunding true {} {}", hour, minute);
    // 如果不在任何funding time，则返回false
    true
}
