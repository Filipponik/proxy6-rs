#[allow(clippy::wildcard_imports)]
use crate::value_object::*;

#[allow(clippy::redundant_pub_crate, reason = "`pub use` this module")]
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
pub struct GetPrice {
    pub count: usize,
    pub period: ProxyPeriod,
    pub version: Option<ProxyVersion>,
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
    pub country: Country,
    pub version: Option<ProxyVersion>,
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
    pub version: Option<ProxyVersion>,
}

impl ApiParams for GetCountry {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![("version", self.version.as_ref().map(ToString::to_string))]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProxy {
    pub state: Option<ProxyStatus>,
    pub description: Option<ProxyDescription>,
    pub page: Option<usize>,
    pub limit: Option<PageLimit>,
}

impl ApiParams for GetProxy {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("state", self.state.as_ref().map(ToString::to_string)),
            ("descr", self.description.as_ref().map(ToString::to_string)),
            ("page", self.page.map(|page| page.to_string())),
            ("limit", self.limit.as_ref().map(ToString::to_string)),
            ("nokey", Some(String::new())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetType {
    pub ids: Vec<ProxyId>,
    pub r#type: ProxyType,
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
    pub new: ProxyDescription,
    pub old: Option<ProxyDescription>, // old or ids is required
    pub ids: Option<Vec<ProxyId>>,
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
    pub count: usize,
    pub period: ProxyPeriod,
    pub country: Country,
    pub version: Option<ProxyVersion>,
    pub r#type: Option<ProxyType>,
    pub description: Option<ProxyDescription>,
    pub auto_prolong: bool,
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
                    Some(String::new())
                } else {
                    None
                },
            ),
            ("nokey", Some(String::new())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prolong {
    pub period: ProxyPeriod,
    pub ids: Vec<ProxyId>,
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
            ("nokey", Some(String::new())),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delete {
    pub ids: Option<Vec<ProxyId>>,
    pub description: Option<ProxyDescription>, // ids or description is required
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
    pub ids: Option<Vec<ProxyId>>,         // plural? wtf?
    pub proxy_string: Option<ProxyString>, // ids of proxy_string is required
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
    pub ip: IpsToConnect,
}

impl ApiParams for IpAuth {
    fn to_query_tuple(&self) -> Vec<(&str, Option<String>)> {
        vec![("ip", Some(self.ip.to_string()))]
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_full_get_price_to_query_string() {
        let request = GetPrice {
            count: 10,
            period: ProxyPeriod::new(30).unwrap(),
            version: Some(ProxyVersion::Ipv6),
        };

        assert_eq!(request.to_query_string(), "count=10&period=30&version=6");
    }

    #[test]
    fn test_convert_minimal_get_price_to_query_string() {
        let request = GetPrice {
            count: 10,
            period: ProxyPeriod::new(30).unwrap(),
            version: None,
        };

        assert_eq!(request.to_query_string(), "count=10&period=30");
    }

    #[test]
    fn test_convert_minimal_get_count_to_query_string() {
        let request = GetCount {
            country: Country::new("uk").unwrap(),
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
            state: Some(ProxyStatus::Active),
            description: Some(ProxyDescription::new("test_description").unwrap()),
            page: Some(3),
            limit: Some(PageLimit::new(10).unwrap()),
        };

        assert_eq!(
            request.to_query_string(),
            "state=active&descr=test_description&page=3&limit=10&nokey"
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

        assert_eq!(request.to_query_string(), "nokey");
    }

    #[test]
    fn test_convert_full_set_type_to_query_string() {
        let request = SetType {
            ids: vec![ProxyId::new("id1"), ProxyId::new("id2")],
            r#type: ProxyType::Socks5,
        };

        assert_eq!(request.to_query_string(), "ids=id1,id2&type=socks");
    }

    #[test]
    fn test_convert_full_set_description_to_query_string() {
        let request = SetDescription {
            new: ProxyDescription::new("new_proxy_description").unwrap(),
            old: Some(ProxyDescription::new("old_proxy_description").unwrap()),
            ids: Some(vec![ProxyId::new("id1"), ProxyId::new("id2")]),
        };

        assert_eq!(
            request.to_query_string(),
            "new=new_proxy_description&old=old_proxy_description&ids=id1,id2"
        );
    }

    #[test]
    fn test_convert_minimal_set_description_to_query_string() {
        let request = SetDescription {
            new: ProxyDescription::new("new_proxy_description").unwrap(),
            old: None,
            ids: None,
        };

        assert_eq!(request.to_query_string(), "new=new_proxy_description");
    }

    #[test]
    fn test_convert_full_buy_to_query_string() {
        let request = Buy {
            count: 100,
            period: ProxyPeriod::new(30).unwrap(),
            country: Country::new("us").unwrap(),
            version: Some(ProxyVersion::Ipv6),
            r#type: Some(ProxyType::Http),
            description: Some(ProxyDescription::new("new_proxy_description").unwrap()),
            auto_prolong: true,
        };

        assert_eq!(
            request.to_query_string(),
            "count=100&period=30&country=us&version=6&type=http&descr=new_proxy_description&auto_prolong&nokey"
        );
    }

    #[test]
    fn test_convert_minimal_buy_to_query_string() {
        let request = Buy {
            count: 100,
            period: ProxyPeriod::new(30).unwrap(),
            country: Country::new("us").unwrap(),
            version: None,
            r#type: None,
            description: None,
            auto_prolong: false,
        };

        assert_eq!(
            request.to_query_string(),
            "count=100&period=30&country=us&nokey"
        );
    }

    #[test]
    fn test_convert_full_prolong_to_query_string() {
        let request = Prolong {
            period: ProxyPeriod::new(30).unwrap(),
            ids: vec![ProxyId::new("id1"), ProxyId::new("id2")],
        };

        assert_eq!(request.to_query_string(), "period=30&ids=id1,id2&nokey");
    }

    #[test]
    fn test_convert_full_delete_to_query_string() {
        let request = Delete {
            ids: Some(vec![ProxyId::new("id1"), ProxyId::new("id2")]),
            description: Some(ProxyDescription::new("new_proxy_description").unwrap()),
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
            ids: Some(vec![ProxyId::new("id1"), ProxyId::new("id2")]),
            proxy_string: Some(ProxyString::new("127.0.0.1:8080:user:pass").unwrap()),
        };

        assert_eq!(
            request.to_query_string(),
            "ids=id1,id2&proxy=127.0.0.1:8080:user:pass"
        );
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
