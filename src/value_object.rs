use std::{fmt::Display, net::IpAddr};

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
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
pub struct ProxyPeriod(#[serde(deserialize_with = "crate::deserializer::to_usize")] usize);

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

    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.0
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        self.0
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    #[must_use]
    pub const fn new(username: String) -> Self {
        Self(username)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserBalance(String);

impl UserBalance {
    #[must_use]
    pub const fn new(balance: String) -> Self {
        Self(balance)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Currency(String);

impl Currency {
    #[must_use]
    pub const fn new(currency: String) -> Self {
        Self(currency)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
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

    #[must_use]
    pub const fn as_f64(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderId(usize);

impl OrderId {
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // ===== ProxyPeriod tests =====

    #[test]
    fn test_proxy_period_new_valid() {
        let period = ProxyPeriod::new(100).unwrap();
        assert_eq!(period.as_usize(), 100);
    }

    #[test]
    fn test_proxy_period_new_one() {
        let period = ProxyPeriod::new(1).unwrap();
        assert_eq!(period.as_usize(), 1);
    }

    #[test]
    fn test_proxy_period_new_large_value() {
        let period = ProxyPeriod::new(999_999).unwrap();
        assert_eq!(period.as_usize(), 999_999);
    }

    #[test]
    fn test_proxy_period_new_zero_error() {
        let result = ProxyPeriod::new(0);
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyPeriodTooLow));
    }

    // ===== Country tests =====

    #[test]
    fn test_country_new_valid_two_letter() {
        let country = Country::new("US").unwrap();
        assert_eq!(country.as_str(), "us");
    }

    #[test]
    fn test_country_new_uppercase_converts_to_lowercase() {
        let country = Country::new("RU").unwrap();
        assert_eq!(country.as_str(), "ru");
    }

    #[test]
    fn test_country_new_already_lowercase() {
        let country = Country::new("de").unwrap();
        assert_eq!(country.as_str(), "de");
    }

    #[test]
    fn test_country_new_one_letter_error() {
        let result = Country::new("U");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::CountryMustBeIso2));
    }

    #[test]
    fn test_country_new_three_letter_error() {
        let result = Country::new("USA");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::CountryMustBeIso2));
    }

    #[test]
    fn test_country_new_empty_error() {
        let result = Country::new("");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::CountryMustBeIso2));
    }

    // ===== PageLimit tests =====

    #[test]
    fn test_page_limit_new_valid() {
        let limit = PageLimit::new(100).unwrap();
        assert_eq!(limit.as_u16(), 100);
    }

    #[test]
    fn test_page_limit_new_one() {
        let limit = PageLimit::new(1).unwrap();
        assert_eq!(limit.as_u16(), 1);
    }

    #[test]
    fn test_page_limit_new_max() {
        let limit = PageLimit::new(1000).unwrap();
        assert_eq!(limit.as_u16(), 1000);
    }

    #[test]
    fn test_page_limit_new_zero_error() {
        let result = PageLimit::new(0);
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::PageLimitTooLow));
    }

    #[test]
    fn test_page_limit_new_too_high_error() {
        let result = PageLimit::new(1001);
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::PageLimitTooHigh));
    }

    // ===== ProxyDescription tests =====

    #[test]
    fn test_proxy_description_new_valid() {
        let desc = ProxyDescription::new("My proxy").unwrap();
        assert_eq!(desc.as_str(), "My proxy");
    }

    #[test]
    fn test_proxy_description_new_empty() {
        let desc = ProxyDescription::new("").unwrap();
        assert_eq!(desc.as_str(), "");
    }

    #[test]
    fn test_proxy_description_new_fifty_chars() {
        let desc = ProxyDescription::new("a".repeat(50)).unwrap();
        assert_eq!(desc.as_str(), "a".repeat(50).as_str());
        assert_eq!(desc.as_str().len(), 50);
    }

    #[test]
    fn test_proxy_description_new_too_long_error() {
        let result = ProxyDescription::new("a".repeat(51));
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyDescriptionTooLong));
    }

    // ===== ProxyId tests =====

    #[test]
    fn test_proxy_id_new_string() {
        let id = ProxyId::new("test-id-123");
        assert_eq!(id.as_str(), "test-id-123");
    }

    #[test]
    fn test_proxy_id_new_empty() {
        let id = ProxyId::new("");
        assert_eq!(id.as_str(), "");
    }

    // ===== ProxyString tests =====

    #[test]
    fn test_proxy_string_new_valid() {
        let proxy = ProxyString::new("192.168.1.1:8080:user:pass").unwrap();
        assert_eq!(proxy.as_str(), "192.168.1.1:8080:user:pass");
    }

    #[test]
    fn test_proxy_string_new_valid_large_port() {
        let proxy = ProxyString::new("192.168.1.1:65535:user:pass").unwrap();
        assert_eq!(proxy.as_str(), "192.168.1.1:65535:user:pass");
    }

    #[test]
    fn test_proxy_string_new_valid_port_zero() {
        let proxy = ProxyString::new("192.168.1.1:0:user:pass").unwrap();
        assert_eq!(proxy.as_str(), "192.168.1.1:0:user:pass");
    }

    #[test]
    fn test_proxy_string_new_invalid_format_missing_parts() {
        let result = ProxyString::new("192.168.1.1:8080:user");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_invalid_format_too_many_parts() {
        let result = ProxyString::new("192.168.1.1:8080:user:pass:extra");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_invalid_ip() {
        let result = ProxyString::new("invalid:8080:user:pass");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_invalid_ip_format() {
        let result = ProxyString::new("192.168.1:8080:user:pass");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_invalid_port_not_number() {
        let result = ProxyString::new("192.168.1.1:abc:user:pass");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_invalid_port_too_high() {
        let result = ProxyString::new("192.168.1.1:65536:user:pass");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_empty_user() {
        let result = ProxyString::new("192.168.1.1:8080::pass");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_empty_pass() {
        let result = ProxyString::new("192.168.1.1:8080:user:");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    #[test]
    fn test_proxy_string_new_empty_user_and_pass() {
        let result = ProxyString::new("192.168.1.1:8080::");
        assert!(result.is_err());
        assert_eq!(result, Err(BuildError::ProxyStringIncorrectFormat));
    }

    // ===== Port tests =====

    #[test]
    fn test_port_new() {
        let port = Port::new(8080);
        assert_eq!(port.as_u16(), 8080);
    }

    #[test]
    fn test_port_new_zero() {
        let port = Port::new(0);
        assert_eq!(port.as_u16(), 0);
    }

    #[test]
    fn test_port_new_max() {
        let port = Port::new(65535);
        assert_eq!(port.as_u16(), 65535);
    }

    // ===== Username tests =====

    #[test]
    fn test_username_new() {
        let username = Username::new("testuser".to_string());
        assert_eq!(username.as_str(), "testuser");
    }

    #[test]
    fn test_username_new_empty() {
        let username = Username::new(String::new());
        assert_eq!(username.as_str(), "");
    }

    // ===== Password tests =====

    #[test]
    fn test_password_new() {
        let password = Password::new("secret123".to_string());
        assert_eq!(password.0, "secret123");
    }

    #[test]
    fn test_password_new_empty() {
        let password = Password::new(String::new());
        assert_eq!(password.0, "");
    }

    // ===== ResponseStatus tests =====

    #[test]
    fn test_response_status_new() {
        let status = ResponseStatus::new("success".to_string());
        assert_eq!(status.as_str(), "success");
    }

    // ===== UserId tests =====

    #[test]
    fn test_user_id_new() {
        let id = UserId::new("user123".to_string());
        assert_eq!(id.as_str(), "user123");
    }

    // ===== UserBalance tests =====

    #[test]
    fn test_user_balance_new() {
        let balance = UserBalance::new("100.50".to_string());
        assert_eq!(balance.as_str(), "100.50");
    }

    // ===== Currency tests =====

    #[test]
    fn test_currency_new() {
        let currency = Currency::new("USD".to_string());
        assert_eq!(currency.as_str(), "USD");
    }

    // ===== Price tests =====

    #[test]
    fn test_price_new() {
        let price = Price::new(9.99);
        assert!((price.as_f64() - 9.99).abs() < f64::EPSILON);
    }

    #[test]
    fn test_price_new_zero() {
        let price = Price::new(0.0);
        assert!((price.as_f64() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_price_new_negative() {
        let price = Price::new(-5.5);
        assert!((price.as_f64() - (-5.5)).abs() < f64::EPSILON);
    }

    // ===== OrderId tests =====

    #[test]
    fn test_order_id_new() {
        let id = OrderId::new(12345);
        assert_eq!(id.as_usize(), 12345);
    }

    #[test]
    fn test_order_id_new_zero() {
        let id = OrderId::new(0);
        assert_eq!(id.as_usize(), 0);
    }

    #[test]
    fn test_order_id_new_large() {
        let id = OrderId::new(18_446_744_073_709_551_615);
        assert_eq!(id.as_usize(), 18_446_744_073_709_551_615);
    }
}
