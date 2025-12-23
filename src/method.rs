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
