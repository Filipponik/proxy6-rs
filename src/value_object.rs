use std::{fmt::Display, net::IpAddr};

use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ProxyPeriod(#[serde(deserialize_with = "crate::deserializer::to_usize")] usize); // Enum needed here? example 30

impl ProxyPeriod {
    /// Create a new `ProxyPeriod` instance.
    ///
    /// # Errors
    /// - [`BuildError::ProxyPeriodTooLow`] if period is zero.
    pub const fn new(period: usize) -> Result<Self> {
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Country(String);

impl Country {
    /// Create a new `Country` instance.
    ///
    /// # Errors
    /// - [`BuildError::CountryMustBeIso2`] if the provided ISO 2-letter code is not valid.
    pub fn new(iso2: impl Into<String>) -> Result<Self> {
        let iso2 = iso2.into();
        if iso2.len() == 2 {
            Ok(Self(iso2.to_lowercase()))
        } else {
            Err(BuildError::CountryMustBeIso2)
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PageLimit(u16);

impl PageLimit {
    /// Create a new `PageLimit` instance.
    ///
    /// # Errors
    /// - [`BuildError::PageLimitTooLow`] if limit is zero.
    /// - [`BuildError::PageLimitTooHigh`] if limit exceeds 1000.
    pub const fn new(limit: u16) -> Result<Self> {
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ProxyDescription(String);

impl ProxyDescription {
    /// Create a new `ProxyDescription` instance.
    ///
    /// # Errors
    /// - [`BuildError::ProxyDescriptionTooLong`] if the description is longer than 50 characters.
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ProxyId(#[serde(deserialize_with = "crate::deserializer::to_string")] String);

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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ProxyString(String);

impl ProxyString {
    /// Create a new `ProxyString` instance.
    ///
    /// # Errors
    /// - [`BuildError::ProxyStringIncorrectFormat`] if the proxy string is not in the correct format:
    ///     - `ip:port:user:pass`;
    ///     - ip must be ip address;
    ///     - port must be u16;
    ///     - user and pass must not be empty.
    pub fn new(proxy_string: impl Into<String>) -> Result<Self> {
        let proxy_string = proxy_string.into();

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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum ProxyType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "socks")]
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum ProxyStatus {
    Active,
    Inactive,
    Expiring,
    All,
}

impl Display for ProxyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Expiring => write!(f, "expiring"),
            Self::All => write!(f, "all"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Port(#[serde(deserialize_with = "crate::deserializer::to_u16")] u16);

impl Port {
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self(port)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    #[must_use]
    pub const fn new(username: String) -> Self {
        Self(username)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    #[must_use]
    pub const fn new(password: String) -> Self {
        Self(password)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ResponseStatus(String);

impl ResponseStatus {
    #[must_use]
    pub const fn new(status: String) -> Self {
        Self(status)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserBalance(String);

impl UserBalance {
    #[must_use]
    pub const fn new(balance: String) -> Self {
        Self(balance)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Currency(String);

impl Currency {
    #[must_use]
    pub const fn new(currency: String) -> Self {
        Self(currency)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Proxy {
    pub id: ProxyId,
    pub ip: IpAddr,
    pub host: IpAddr,
    pub port: Port,
    pub user: Username,
    #[serde(rename = "pass")]
    pub password: Password,
    pub r#type: ProxyType,
    pub country: Country,
    pub date: String,     // use chrono
    pub date_end: String, // use chrono
    pub unixtime: u64,
    pub unixtime_end: u64,
    #[serde(rename = "descr")]
    pub description: ProxyDescription,
    #[serde(deserialize_with = "crate::deserializer::parse_proxy_status")]
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Price(#[serde(deserialize_with = "crate::deserializer::to_f64")] f64);

impl Price {
    #[must_use]
    pub const fn new(price: f64) -> Self {
        Self(price)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderId(usize);

impl OrderId {
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}
