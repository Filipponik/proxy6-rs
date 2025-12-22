use std::net::IpAddr;

pub(crate) trait ApiParams {
    fn to_query_string(&self) -> String;
}

pub struct ProxyPeriod(usize); // Enum needed here? example 30
pub struct Country(String); // 2 symbols
pub struct PageLimit(u16); // 0 < limit <= 1000
pub struct ProxyDescription(String); // <= 50 symbols
pub struct ProxyId(String);
pub struct ProxyString(String); // format: `ip:port:user:pass`

pub enum IpsToConnect {
    Delete,
    Connect(Vec<IpAddr>),
}

pub enum ProxyType {
    Http,
    Socks5,
}

impl ApiParams for ProxyType {
    fn to_query_string(&self) -> String {
        match self {
            ProxyType::Http => "http",
            ProxyType::Socks5 => "socks",
        }
        .to_string()
    }
}

pub enum State {
    Active,
    Inactive,
    Expiring,
    All,
}

impl ApiParams for State {
    fn to_query_string(&self) -> String {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Expiring => "expiring",
            Self::All => "all",
        }
        .to_string()
    }
}

pub enum ProxyVersion {
    Ipv4,
    Ipv6,
    Ipv4Shared,
}

impl ApiParams for ProxyVersion {
    fn to_query_string(&self) -> String {
        match self {
            ProxyVersion::Ipv4 => 4,
            ProxyVersion::Ipv6 => 6,
            ProxyVersion::Ipv4Shared => 3,
        }
        .to_string()
    }
}

pub struct GetPrice {
    count: usize,
    period: ProxyPeriod,
    version: Option<ProxyVersion>,
}

pub struct GetCount {
    country: Country,
    version: Option<ProxyVersion>,
}

pub struct GetCountry {
    version: Option<ProxyVersion>,
}

pub struct GetProxy {
    state: Option<State>,
    description: Option<ProxyDescription>,
    page: Option<usize>,
    limit: Option<PageLimit>,
}

pub struct SetType {
    ids: Vec<ProxyId>,
    r#type: ProxyType,
}

pub struct SetDescription {
    new: ProxyDescription,
    old: Option<ProxyDescription>, // old or ids is required
    ids: Option<Vec<ProxyId>>,
}

pub struct BuyProxy {
    count: usize,
    period: ProxyPeriod,
    country: Country,
    version: Option<ProxyVersion>,
    r#type: Option<ProxyType>,
    description: Option<ProxyDescription>,
    auto_prolong: bool,
}

pub struct Prolong {
    period: ProxyPeriod,
    ids: Vec<ProxyId>,
}

pub struct Delete {
    ids: Option<Vec<ProxyId>>,
    description: Option<ProxyDescription>, // ids or description is required
}

pub struct Check {
    ids: Option<Vec<ProxyId>>,         // plural? wtf?
    proxy_string: Option<ProxyString>, // ids of proxy_string is required
}

pub struct IpAuth {
    ip: IpsToConnect,
}
