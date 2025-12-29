use std::fmt::Display;

use crate::params;

#[derive(Debug, Clone)]
pub enum ApiMethod {
    GetPrice(params::GetPrice),
    GetCount(params::GetCount),
    GetCountry(params::GetCountry),
    GetProxy(params::GetProxy),
    SetType(params::SetType),
    SetDescription(params::SetDescription),
    Buy(params::Buy),
    Prolong(params::Prolong),
    Delete(params::Delete),
    Check(params::Check),
    IpAuth(params::IpAuth),
}

impl ApiMethod {
    pub fn get_params(&self) -> &dyn params::ApiParams {
        match self {
            Self::GetPrice(params) => params,
            Self::GetCount(params) => params,
            Self::GetCountry(params) => params,
            Self::GetProxy(params) => params,
            Self::SetType(params) => params,
            Self::SetDescription(params) => params,
            Self::Buy(params) => params,
            Self::Prolong(params) => params,
            Self::Delete(params) => params,
            Self::Check(params) => params,
            Self::IpAuth(params) => params,
        }
    }
}

impl Display for ApiMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetPrice(_) => "getprice",
            Self::GetCount(_) => "getcount",
            Self::GetCountry(_) => "getcountry",
            Self::GetProxy(_) => "getproxy",
            Self::SetType(_) => "settype",
            Self::SetDescription(_) => "setdescr",
            Self::Buy(_) => "buy",
            Self::Prolong(_) => "prolong",
            Self::Delete(_) => "delete",
            Self::Check(_) => "check",
            Self::IpAuth(_) => "ipauth",
        }
        .fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::params;

    #[test]
    fn test_get_price_display() {
        let params = params::GetPrice {
            count: 10,
            period: crate::value_object::ProxyPeriod::new(30).unwrap(),
            version: Some(crate::value_object::ProxyVersion::Ipv6),
        };
        let method = ApiMethod::GetPrice(params);

        assert_eq!(method.to_string(), "getprice");
    }

    #[test]
    fn test_get_count_display() {
        let params = params::GetCount {
            country: crate::value_object::Country::new("us").unwrap(),
            version: None,
        };
        let method = ApiMethod::GetCount(params);

        assert_eq!(method.to_string(), "getcount");
    }

    #[test]
    fn test_get_country_display() {
        let params = params::GetCountry {
            version: Some(crate::value_object::ProxyVersion::Ipv4),
        };
        let method = ApiMethod::GetCountry(params);

        assert_eq!(method.to_string(), "getcountry");
    }

    #[test]
    fn test_get_proxy_display() {
        let params = params::GetProxy {
            state: None,
            description: None,
            page: None,
            limit: None,
        };
        let method = ApiMethod::GetProxy(params);

        assert_eq!(method.to_string(), "getproxy");
    }

    #[test]
    fn test_set_type_display() {
        let params = params::SetType {
            ids: vec![crate::value_object::ProxyId::new("id1")],
            r#type: crate::value_object::ProxyType::Http,
        };
        let method = ApiMethod::SetType(params);

        assert_eq!(method.to_string(), "settype");
    }

    #[test]
    fn test_set_description_display() {
        let params = params::SetDescription {
            new: crate::value_object::ProxyDescription::new("test").unwrap(),
            old: None,
            ids: None,
        };
        let method = ApiMethod::SetDescription(params);

        assert_eq!(method.to_string(), "setdescr");
    }

    #[test]
    fn test_buy_display() {
        let params = params::Buy {
            count: 1,
            period: crate::value_object::ProxyPeriod::new(30).unwrap(),
            country: crate::value_object::Country::new("us").unwrap(),
            version: None,
            r#type: None,
            description: None,
            auto_prolong: false,
        };
        let method = ApiMethod::Buy(params);

        assert_eq!(method.to_string(), "buy");
    }

    #[test]
    fn test_prolong_display() {
        let params = params::Prolong {
            period: crate::value_object::ProxyPeriod::new(30).unwrap(),
            ids: vec![crate::value_object::ProxyId::new("id1")],
        };
        let method = ApiMethod::Prolong(params);

        assert_eq!(method.to_string(), "prolong");
    }

    #[test]
    fn test_delete_display() {
        let params = params::Delete {
            ids: None,
            description: None,
        };
        let method = ApiMethod::Delete(params);

        assert_eq!(method.to_string(), "delete");
    }

    #[test]
    fn test_check_display() {
        let params = params::Check {
            ids: None,
            proxy_string: None,
        };
        let method = ApiMethod::Check(params);

        assert_eq!(method.to_string(), "check");
    }

    #[test]
    fn test_ip_auth_display() {
        let params = params::IpAuth {
            ip: crate::value_object::IpsToConnect::Delete,
        };
        let method = ApiMethod::IpAuth(params);

        assert_eq!(method.to_string(), "ipauth");
    }

    #[test]
    fn test_get_params() {
        let params = params::GetPrice {
            count: 10,
            period: crate::value_object::ProxyPeriod::new(30).unwrap(),
            version: Some(crate::value_object::ProxyVersion::Ipv6),
        };
        let method = ApiMethod::GetPrice(params.clone());

        // Возвращаемая ссылка имеет тип &dyn ApiParams, проверяем, что она работает
        let api_params = method.get_params();
        assert_eq!(api_params.to_query_string(), "count=10&period=30&version=6");

        // Проверка, что это правильный тип через сопоставление с образцом на enum ApiMethod
        match method {
            ApiMethod::GetPrice(price_params) => {
                assert_eq!(price_params.count, params.count);
                assert_eq!(price_params.period.as_usize(), params.period.as_usize());
                assert_eq!(price_params.version, params.version);
            }
            _ => panic!("Expected GetPrice method"),
        }
    }
}
