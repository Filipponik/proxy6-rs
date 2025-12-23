use std::{fmt::Display, net::IpAddr};

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Proxy period must be greater than zero")]
    ProxyPeriodTooLow,
    #[error("Country must be ISO2 format")]
    CountryMustBeIso2,
    #[error("Page limit must be greater than zero")]
    PageLimitTooLow,
    #[error("Page limit must be less than or equal to 1000")]
    PageLimitTooHigh,
    #[error("Proxy description must be less than or equal to 50 symbols")]
    ProxyDescriptionTooLong,
    #[error("Proxy string format must be `ip:port:user:pass`, user and password must be non-empty")]
    ProxyStringIncorrectFormat,
}

type Result<T> = std::result::Result<T, BuildError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyPeriod(usize); // Enum needed here? example 30

impl ProxyPeriod {
    pub fn new(period: usize) -> Result<Self> {
        if period == 0 {
            Err(BuildError::ProxyPeriodTooLow)
        } else {
            Ok(Self(period))
        }
    }
}

impl Display for ProxyPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Country(String);

impl Country {
    pub fn new(iso2: impl Into<String>) -> Result<Self> {
        let iso2 = iso2.into();
        if iso2.len() != 2 {
            Err(BuildError::CountryMustBeIso2)
        } else {
            Ok(Self(iso2.to_lowercase()))
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLimit(u16);

impl PageLimit {
    pub fn new(limit: u16) -> Result<Self> {
        if limit == 0 {
            Err(BuildError::PageLimitTooLow)
        } else if limit > 1000 {
            Err(BuildError::PageLimitTooHigh)
        } else {
            Ok(Self(limit))
        }
    }
}

impl Display for PageLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyDescription(String);

impl ProxyDescription {
    pub fn new(description: impl Into<String>) -> Result<Self> {
        let description = description.into();
        if description.len() > 50 {
            Err(BuildError::ProxyDescriptionTooLong)
        } else {
            Ok(Self(description))
        }
    }
}

impl Display for ProxyDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyId(String);

impl ProxyId {
    pub fn new(proxy_id: impl Into<String>) -> Self {
        Self(proxy_id.into())
    }
}

impl Display for ProxyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyString(String);

impl ProxyString {
    pub fn new(proxy_string: impl Into<String>) -> Result<Self> {
        let proxy_string = proxy_string.into();
        // check format ip:port:user:pass

        let parts: Vec<&str> = proxy_string.split(':').collect();
        if parts.len() != 4 {
            return Err(BuildError::ProxyStringIncorrectFormat);
        }

        let ip = parts[0];
        let port = parts[1];
        let user = parts[2];
        let pass = parts[3];

        if ip.parse::<IpAddr>().is_err() {
            return Err(BuildError::ProxyStringIncorrectFormat);
        }

        if port.parse::<u16>().is_err() {
            return Err(BuildError::ProxyStringIncorrectFormat);
        }

        if user.is_empty() || pass.is_empty() {
            return Err(BuildError::ProxyStringIncorrectFormat);
        }

        Ok(Self(proxy_string))
    }
}

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
