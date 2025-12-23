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
    pub fn get_params(&self) -> Box<&dyn params::ApiParams> {
        match self {
            Self::GetPrice(params) => Box::new(params),
            Self::GetCount(params) => Box::new(params),
            Self::GetCountry(params) => Box::new(params),
            Self::GetProxy(params) => Box::new(params),
            Self::SetType(params) => Box::new(params),
            Self::SetDescription(params) => Box::new(params),
            Self::Buy(params) => Box::new(params),
            Self::Prolong(params) => Box::new(params),
            Self::Delete(params) => Box::new(params),
            Self::Check(params) => Box::new(params),
            Self::IpAuth(params) => Box::new(params),
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
