use std::{fmt::Display, net::IpAddr};

pub(crate) trait ApiParams {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)>;

    fn to_query_string(&self) -> String {
        self.to_query_tuple()
            .into_iter()
            .filter_map(|(key, maybe_value)| {
                let Some(value) = maybe_value else {
                    return None;
                };

                if value.is_empty() {
                    return Some(key.to_string());
                }

                Some(format!("{}={}", key, value))
            })
            .collect::<Vec<_>>()
            .join("&");

        String::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyPeriod(usize); // Enum needed here? example 30

impl Display for ProxyPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Country(String); // 2 symbols

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLimit(u16); // 0 < limit <= 1000

impl Display for PageLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyDescription(String); // <= 50 symbols

impl Display for ProxyDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyId(String);

impl Display for ProxyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyString(String); // format: `ip:port:user:pass`

impl Display for ProxyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpsToConnect {
    Delete,
    Connect(Vec<IpAddr>),
}

impl Display for IpsToConnect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpsToConnect::Delete => write!(f, "delete"),
            IpsToConnect::Connect(ips) => {
                let ips = ips.iter().map(|ip| ip.to_string()).collect::<Vec<_>>();
                write!(f, "{}", ips.join(","))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyType {
    Http,
    Socks5,
}

impl Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyType::Http => write!(f, "http"),
            ProxyType::Socks5 => write!(f, "socks"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Active,
    Inactive,
    Expiring,
    All,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Expiring => write!(f, "expiring"),
            Self::All => write!(f, "all"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyVersion {
    Ipv4,
    Ipv6,
    Ipv4Shared,
}

impl Display for ProxyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyVersion::Ipv4 => write!(f, "4"),
            ProxyVersion::Ipv6 => write!(f, "6"),
            ProxyVersion::Ipv4Shared => write!(f, "3"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPrice {
    count: usize,
    period: ProxyPeriod,
    version: Option<ProxyVersion>,
}

impl ApiParams for GetPrice {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("count", Some(self.count.to_string())),
            ("period", Some(self.period.to_string())),
            ("version", self.version.as_ref().map(|v| v.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCount {
    country: Country,
    version: Option<ProxyVersion>,
}

impl ApiParams for GetCount {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("country", Some(self.country.to_string())),
            ("version", self.version.as_ref().map(|v| v.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCountry {
    version: Option<ProxyVersion>,
}

impl ApiParams for GetCountry {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![("version", self.version.as_ref().map(|v| v.to_string()))]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProxy {
    state: Option<State>,
    description: Option<ProxyDescription>,
    page: Option<usize>,
    limit: Option<PageLimit>,
}

impl ApiParams for GetProxy {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("state", self.state.as_ref().map(|s| s.to_string())),
            (
                "description",
                self.description.as_ref().map(|d| d.to_string()),
            ),
            ("page", self.page.map(|p| p.to_string())),
            ("limit", self.limit.as_ref().map(|l| l.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetType {
    ids: Vec<ProxyId>,
    r#type: ProxyType,
}

impl ApiParams for SetType {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            (
                "ids",
                Some(
                    self.ids
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ),
            ("type", Some(self.r#type.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDescription {
    new: ProxyDescription,
    old: Option<ProxyDescription>, // old or ids is required
    ids: Option<Vec<ProxyId>>,
}

impl ApiParams for SetDescription {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("new", Some(self.new.to_string())),
            ("old", self.old.as_ref().map(|old| old.to_string())),
            (
                "ids",
                self.ids.as_ref().map(|ids| {
                    ids.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCountry {
    ids: Vec<ProxyId>,
    country: Country,
}

impl ApiParams for SetCountry {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            (
                "ids",
                Some(
                    self.ids
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ),
            ("country", Some(self.country.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buy {
    count: usize,
    period: ProxyPeriod,
    country: Country,
    version: Option<ProxyVersion>,
    r#type: Option<ProxyType>,
    description: Option<ProxyDescription>,
    auto_prolong: bool,
}

impl ApiParams for Buy {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("count", Some(self.count.to_string())),
            ("period", Some(self.period.to_string())),
            ("country", Some(self.country.to_string())),
            ("version", self.version.as_ref().map(|x| x.to_string())),
            ("type", self.r#type.as_ref().map(|x| x.to_string())),
            (
                "description",
                self.description.as_ref().map(|x| x.to_string()),
            ),
            ("auto_prolong", Some(self.auto_prolong.to_string())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prolong {
    period: ProxyPeriod,
    ids: Vec<ProxyId>,
}

impl ApiParams for Prolong {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("period", Some(self.period.to_string())),
            (
                "ids",
                Some(
                    self.ids
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delete {
    ids: Option<Vec<ProxyId>>,
    description: Option<ProxyDescription>, // ids or description is required
}

impl ApiParams for Delete {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            (
                "ids",
                self.ids.as_ref().map(|ids| {
                    ids.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
            (
                "description",
                self.description.as_ref().map(|x| x.to_string()),
            ),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    ids: Option<Vec<ProxyId>>,         // plural? wtf?
    proxy_string: Option<ProxyString>, // ids of proxy_string is required
}

impl ApiParams for Check {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            (
                "ids",
                self.ids.as_ref().map(|ids| {
                    ids.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
            (
                "proxy_string",
                self.proxy_string.as_ref().map(|x| x.to_string()),
            ),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpAuth {
    ip: IpsToConnect,
}

impl ApiParams for IpAuth {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![("ip", Some(self.ip.to_string()))]
    }
}
