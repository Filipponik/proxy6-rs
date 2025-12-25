use std::net::IpAddr;

use serde::Deserialize;

#[allow(clippy::wildcard_imports)]
use crate::value_object::*;

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPrice {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub price: Price,
    pub price_single: Price,
    pub period: ProxyPeriod,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCount {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCountry {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub list: Vec<Country>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetProxy {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub list_count: usize,
    pub list: Vec<Proxy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetDescription {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Buy {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub order_id: OrderId,
    #[serde(deserialize_with = "crate::deserializer::to_usize")]
    pub count: usize,
    pub price: Price,
    pub period: ProxyPeriod,
    pub country: String,
    pub list: Vec<BoughtProxy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProlongedProxy {
    pub id: ProxyId,
    pub date_end: String, // use chrono
    pub unixtime_end: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoughtProxy {
    pub id: ProxyId,
    pub ip: IpAddr,
    pub host: IpAddr,
    pub port: Port,
    pub user: Username,
    #[serde(rename = "pass")]
    pub password: Password,
    pub r#type: ProxyType,
    pub date: String,     // use chrono
    pub date_end: String, // use chrono
    pub unixtime: u64,
    pub unixtime_end: u64,
    #[serde(deserialize_with = "crate::deserializer::parse_proxy_status")]
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Prolong {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub order_id: OrderId,
    pub price: Price,
    pub period: ProxyPeriod,
    #[serde(deserialize_with = "crate::deserializer::to_usize")]
    pub count: usize,
    pub list: Vec<ProlongedProxy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Delete {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Check {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub proxy_id: Option<ProxyId>,
    pub proxy_status: bool,
    pub proxy_time: f64,
}
