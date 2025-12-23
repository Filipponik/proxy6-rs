use std::fmt::Display;

use crate::params;

#[derive(Debug, Clone)]
pub(crate) enum ApiMethod {
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
    pub(crate) fn get_params(self) -> Box<dyn params::ApiParams> {
        match self {
            ApiMethod::GetPrice(params) => Box::new(params),
            ApiMethod::GetCount(params) => Box::new(params),
            ApiMethod::GetCountry(params) => Box::new(params),
            ApiMethod::GetProxy(params) => Box::new(params),
            ApiMethod::SetType(params) => Box::new(params),
            ApiMethod::SetDescription(params) => Box::new(params),
            ApiMethod::Buy(params) => Box::new(params),
            ApiMethod::Prolong(params) => Box::new(params),
            ApiMethod::Delete(params) => Box::new(params),
            ApiMethod::Check(params) => Box::new(params),
            ApiMethod::IpAuth(params) => Box::new(params),
        }
    }
}

impl Display for ApiMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiMethod::GetPrice(_) => "getprice",
            ApiMethod::GetCount(_) => "getcount",
            ApiMethod::GetCountry(_) => "getcountry",
            ApiMethod::GetProxy(_) => "getproxy",
            ApiMethod::SetType(_) => "settype",
            ApiMethod::SetDescription(_) => "setdescr",
            ApiMethod::Buy(_) => "buy",
            ApiMethod::Prolong(_) => "prolong",
            ApiMethod::Delete(_) => "delete",
            ApiMethod::Check(_) => "check",
            ApiMethod::IpAuth(_) => "ipauth",
        }
        .fmt(f)
    }
}
