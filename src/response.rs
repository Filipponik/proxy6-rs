use crate::value_object::*;

pub struct SuccessResponse {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
}

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

pub struct GetCount {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

pub struct GetCountry {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub list: Vec<Country>,
}

pub struct GetProxy {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub list_count: usize,
    pub list: Vec<Proxy>,
}

pub struct SetDescription {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

pub struct Buy {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub order_id: OrderId,
    pub count: usize,
    pub price: Price,
    pub period: ProxyPeriod,
    pub country: String,
    pub list: String,
}

pub struct ProlongedProxy {
    pub id: ProxyId,
    pub date_end: String, // use chrono
    pub unixtime_end: u64,
}

pub struct Prolong {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub order_id: OrderId,
    pub price: Price,
    pub period: ProxyPeriod,
    pub count: usize,
    pub list: Vec<ProlongedProxy>,
}

pub struct Delete {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub count: usize,
}

pub struct Check {
    pub status: ResponseStatus,
    pub user_id: UserId,
    pub balance: UserBalance,
    pub currency: Currency,
    pub proxy_id: ProxyId,
    pub proxy_status: bool,
}
