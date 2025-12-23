use std::{fmt::Display, net::IpAddr};

pub(crate) trait ApiParams {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)>;

    fn to_query_string(&self) -> String {
        self.to_query_tuple()
            .into_iter()
            .filter_map(|(key, maybe_value)| {
                let value = maybe_value?;

                if value.is_empty() {
                    return Some(key.to_string());
                }

                Some(format!("{key}={value}"))
            })
            .collect::<Vec<_>>()
            .join("&")
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
            Self::Delete => write!(f, "delete"),
            Self::Connect(ips) => {
                let ips = ips.iter().map(ToString::to_string).collect::<Vec<_>>();
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
            Self::Http => write!(f, "http"),
            Self::Socks5 => write!(f, "socks"),
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
            Self::Ipv4 => write!(f, "4"),
            Self::Ipv6 => write!(f, "6"),
            Self::Ipv4Shared => write!(f, "3"),
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
            ("version", self.version.as_ref().map(ToString::to_string)),
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
            ("version", self.version.as_ref().map(ToString::to_string)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCountry {
    version: Option<ProxyVersion>,
}

impl ApiParams for GetCountry {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![("version", self.version.as_ref().map(ToString::to_string))]
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
            ("state", self.state.as_ref().map(ToString::to_string)),
            ("descr", self.description.as_ref().map(ToString::to_string)),
            ("page", self.page.map(|page| page.to_string())),
            ("limit", self.limit.as_ref().map(ToString::to_string)),
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
                        .map(ToString::to_string)
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
            ("old", self.old.as_ref().map(ToString::to_string)),
            (
                "ids",
                self.ids.as_ref().map(|ids| {
                    ids.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
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
            ("version", self.version.as_ref().map(ToString::to_string)),
            ("type", self.r#type.as_ref().map(ToString::to_string)),
            ("descr", self.description.as_ref().map(ToString::to_string)),
            (
                "auto_prolong",
                if self.auto_prolong {
                    Some("".to_string())
                } else {
                    None
                },
            ),
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
                        .map(ToString::to_string)
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
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
            ("descr", self.description.as_ref().map(ToString::to_string)),
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
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
            ("proxy", self.proxy_string.as_ref().map(ToString::to_string)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_full_get_price_to_query_string() {
        let request = GetPrice {
            count: 10,
            period: ProxyPeriod(30),
            version: Some(ProxyVersion::Ipv6),
        };

        assert_eq!(request.to_query_string(), "count=10&period=30&version=6");
    }

    #[test]
    fn test_convert_minimal_get_price_to_query_string() {
        let request = GetPrice {
            count: 10,
            period: ProxyPeriod(30),
            version: None,
        };

        assert_eq!(request.to_query_string(), "count=10&period=30");
    }

    #[test]
    fn test_convert_minimal_get_count_to_query_string() {
        let request = GetCount {
            country: Country("uk".to_string()),
            version: None,
        };

        assert_eq!(request.to_query_string(), "country=uk");
    }

    #[test]
    fn test_convert_full_get_country_to_query_string() {
        let request = GetCountry {
            version: Some(ProxyVersion::Ipv6),
        };

        assert_eq!(request.to_query_string(), "version=6");
    }

    #[test]
    fn test_convert_minimal_get_country_to_query_string() {
        let request = GetCountry { version: None };

        assert_eq!(request.to_query_string(), "");
    }

    #[test]
    fn test_convert_full_get_proxy_to_query_string() {
        let request = GetProxy {
            state: Some(State::Active),
            description: Some(ProxyDescription("test_description".to_string())),
            page: Some(3),
            limit: Some(PageLimit(10)),
        };

        assert_eq!(
            request.to_query_string(),
            "state=active&descr=test_description&page=3&limit=10"
        );
    }

    #[test]
    fn test_convert_minimal_get_proxy_to_query_string() {
        let request = GetProxy {
            state: None,
            description: None,
            page: None,
            limit: None,
        };

        assert_eq!(request.to_query_string(), "");
    }

    #[test]
    fn test_convert_full_set_type_to_query_string() {
        let request = SetType {
            ids: vec![ProxyId("id1".to_string()), ProxyId("id2".to_string())],
            r#type: ProxyType::Socks5,
        };

        assert_eq!(request.to_query_string(), "ids=id1,id2&type=socks");
    }

    #[test]
    fn test_convert_full_set_description_to_query_string() {
        let request = SetDescription {
            new: ProxyDescription("new_proxy_description".to_string()),
            old: Some(ProxyDescription("old_proxy_description".to_string())),
            ids: Some(vec![ProxyId("id1".to_string()), ProxyId("id2".to_string())]),
        };

        assert_eq!(
            request.to_query_string(),
            "new=new_proxy_description&old=old_proxy_description&ids=id1,id2"
        );
    }

    #[test]
    fn test_convert_minimal_set_description_to_query_string() {
        let request = SetDescription {
            new: ProxyDescription("new_proxy_description".to_string()),
            old: None,
            ids: None,
        };

        assert_eq!(request.to_query_string(), "new=new_proxy_description");
    }

    #[test]
    fn test_convert_full_buy_to_query_string() {
        let request = Buy {
            count: 100,
            period: ProxyPeriod(30),
            country: Country("us".to_string()),
            version: Some(ProxyVersion::Ipv6),
            r#type: Some(ProxyType::Http),
            description: Some(ProxyDescription("new_proxy_description".to_string())),
            auto_prolong: true,
        };

        assert_eq!(
            request.to_query_string(),
            "count=100&period=30&country=us&version=6&type=http&descr=new_proxy_description&auto_prolong"
        );
    }

    #[test]
    fn test_convert_minimal_buy_to_query_string() {
        let request = Buy {
            count: 100,
            period: ProxyPeriod(30),
            country: Country("us".to_string()),
            version: None,
            r#type: None,
            description: None,
            auto_prolong: false,
        };

        assert_eq!(request.to_query_string(), "count=100&period=30&country=us");
    }

    #[test]
    fn test_convert_full_prolong_to_query_string() {
        let request = Prolong {
            period: ProxyPeriod(30),
            ids: vec![ProxyId("id1".to_string()), ProxyId("id2".to_string())],
        };

        assert_eq!(request.to_query_string(), "period=30&ids=id1,id2");
    }

    #[test]
    fn test_convert_full_delete_to_query_string() {
        let request = Delete {
            ids: Some(vec![ProxyId("id1".to_string()), ProxyId("id2".to_string())]),
            description: Some(ProxyDescription("new_proxy_description".to_string())),
        };

        assert_eq!(
            request.to_query_string(),
            "ids=id1,id2&descr=new_proxy_description"
        );
    }

    #[test]
    fn test_convert_minimal_delete_to_query_string() {
        let request = Delete {
            ids: None,
            description: None,
        };

        assert_eq!(request.to_query_string(), "");
    }

    #[test]
    fn test_convert_full_check_to_query_string() {
        let request = Check {
            ids: Some(vec![ProxyId("id1".to_string()), ProxyId("id2".to_string())]),
            proxy_string: Some(ProxyString("proxy_string".to_string())),
        };

        assert_eq!(request.to_query_string(), "ids=id1,id2&proxy=proxy_string");
    }

    #[test]
    fn test_convert_minimal_check_to_query_string() {
        let request = Check {
            ids: None,
            proxy_string: None,
        };

        assert_eq!(request.to_query_string(), "");
    }

    #[test]
    fn test_convert_delete_ip_auth_to_query_string() {
        let request = IpAuth {
            ip: IpsToConnect::Delete,
        };

        assert_eq!(request.to_query_string(), "ip=delete");
    }

    #[test]
    fn test_convert_select_ip_auth_to_query_string() {
        let request = IpAuth {
            ip: IpsToConnect::Connect(vec![
                "127.0.0.1".parse().unwrap(),
                "127.0.0.2".parse().unwrap(),
            ]),
        };

        assert_eq!(request.to_query_string(), "ip=127.0.0.1,127.0.0.2");
    }
}
