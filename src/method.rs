use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ApiMethod {
    GetPrice,
    GetCount,
    GetCountry,
    GetProxy,
    SetType,
    SetDescription,
    Buy,
    Prolong,
    Delete,
    Check,
    IpAuth,
}

impl Display for ApiMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiMethod::GetPrice => "getprice",
            ApiMethod::GetCount => "getcount",
            ApiMethod::GetCountry => "getcountry",
            ApiMethod::GetProxy => "getproxy",
            ApiMethod::SetType => "settype",
            ApiMethod::SetDescription => "setdescr",
            ApiMethod::Buy => "buy",
            ApiMethod::Prolong => "prolong",
            ApiMethod::Delete => "delete",
            ApiMethod::Check => "check",
            ApiMethod::IpAuth => "ipauth",
        }
        .fmt(f)
    }
}
