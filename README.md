# 🚀 Proxy6 Rust API Wrapper

[![Rust](https://img.shields.io/badge/rust-1.70+-brightgreen.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/github/v/release/Filipponik/proxy6-rs?sort=semver&label=version&color=brightgreen)](https://github.com/Filipponik/proxy6-rs/releases/latest)
[![License](https://img.shields.io/github/license/Filipponik/proxy6-rs?color=brightgreen)](LICENSE)
[![CI - Tests](https://github.com/Filipponik/proxy6-rs/actions/workflows/rust.yml/badge.svg?branch=master&event=push)](https://github.com/Filipponik/proxy6-rs/actions/workflows/rust.yml)
[![Last Commit](https://img.shields.io/github/last-commit/Filipponik/proxy6-rs)](https://github.com/Filipponik/proxy6-rs/commits)

A modern, type-safe Rust client for the [Proxy6](https://proxy6.net/) API. This library provides a complete wrapper for managing proxies, handling authentication, and performing all operations available through the Proxy6 API.

## ✨ Features

- **🔒 Type Safety** - Strongly typed API with compile-time guarantees
- **🚀 Async/Await** - Built on `reqwest` for high-performance async or sync operations
- **📚 Comprehensive** - Complete coverage of all Proxy6 API methods
- **🛡️ Error Handling** - Detailed error types with proper error categorization
- **🧪 Well-Tested** - Extensive test suite with mock server support
- **📖 Documentation** - Full API documentation with examples

## 📦 Installation

Add to `Cargo.toml` or add to project:

```shell
cargo add proxy6
```

## 🚀 Quick Start

```rust
use proxy6::{AsyncClient, Country, params::*};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client with your API key
    let client = AsyncClient::builder().api_key("your-api-key-here").build()?;

    // Get available countries
    let countries = client.get_country(GetCountry { version: None }).await?;
    println!("Available countries: {:?}", countries.list);

    // Get proxy count for a specific country
    let count = client
        .get_count(GetCount {
            country: Country::new("us")?,
            version: None,
        })
        .await?;
    println!("Available proxies in US: {}", count.count);

    // Get your proxy list
    let proxies = client
        .get_proxy(GetProxy {
            state: None,
            description: None,
            page: None,
            limit: None,
        })
        .await?;
    println!("Your proxies: {:?}", proxies.list);

    Ok(())
}
```

## 📋 API Methods

### Proxy Management
- **`get_proxy()`** - Retrieve your proxy list
- **`buy()`** - Purchase new proxies
- **`prolong()`** - Extend proxy validity
- **`delete()`** - Delete proxies
- **`check()`** - Check proxy validity

### Information & Pricing
- **`get_price()`** - Get pricing information
- **`get_count()`** - Get available proxy count by country
- **`get_country()`** - Get available countries

### Proxy Configuration
- **`set_type()`** - Change proxy protocol (HTTP/SOCKS)
- **`set_description()`** - Update proxy descriptions
- **`ip_auth()`** - Manage IP authentication

## 🔧 Advanced Usage

### Error Handling

The library provides detailed error types:

```rust
match client.get_proxy(/* params */).await {
    Ok(response) => println!("Success: {:?}", response),
    Err(ApiError::DocumentedError { code, response }) => {
        eprintln!("API error {}: {}", code, response);
    }
    Err(ApiError::TooManyRequests { response }) => {
        eprintln!("Rate limited: {}", response);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

### Custom HTTP Client

You can provide your own `reqwest::Client` instance (or `reqwest::blocking::Client`):

```rust
// Async client
let reqwest_async_client = reqwest::ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(30))
    .proxy(reqwest::Proxy::all("user:pass@127.0.0.1:8123")?)
    .build()?;

let async_client = proxy6::AsyncClient::builder()
    .api_key("your-api-key")
    .requester(custom_client)
    .build()?;

// Sync client
let reqwest_sync_client = reqwest::blocking::ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(30))
    .proxy(reqwest::Proxy::all("user:pass@127.0.0.1:8123")?)
    .build()?;

let sync_client = proxy6::SyncClient::builder()
    .api_key("your-api-key")
    .requester(custom_client)
    .build()?;
```

### Batch Operations

```rust
// Buy multiple proxies
let buy_response = client
    .buy(Buy {
        count: 5,
        period: ProxyPeriod::new(30)?,
        country: Country::new("us")?,
        version: Some(ProxyVersion::Ipv4),
        r#type: Some(ProxyType::Http),
        description: Some(ProxyDescription::new("my-proxies")?),
        auto_prolong: true,
    })
    .await?;

// Extend specific proxies
let prolong_response = client
    .prolong(Prolong {
        period: ProxyPeriod::new(30)?,
        ids: vec![
            ProxyId::new("proxy-id-1"),
            ProxyId::new("proxy-id-2"),
        ],
    })
    .await?;
```

## 🧪 Testing

The library includes comprehensive tests. Run them with:

```bash
cargo test
```

For tests with mock server:

```bash
cargo test --features mock
```

## 📚 Documentation

Full API documentation is available at:

- [**Docs.rs**](https://docs.rs/proxy6) - Online documentation
- **Local docs**: `cargo doc --open`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Filipponik/proxy6-rs
cd proxy6-rs

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Proxy6 API Documentation](https://px6.me/developers)
- [Crates.io](https://crates.io/crates/proxy6)
- [GitHub Repository](https://github.com/Filipponik/proxy6-rs)

## 🙏 Acknowledgments

- [Proxy6](https://proxy6.net/) for providing the API
- The Rust community for excellent tooling and libraries

---

**Need help?** Open an issue on GitHub or check the comprehensive documentation! 🎯

---

## 👤 Author

**Filipponik**

- GitHub: [@Filipponik](https://github.com/Filipponik)
- Repository: [proxy6-rs](https://github.com/Filipponik/proxy6-rs)

---

<p align="center">Made with ❤️ and 🦀 Rust</p>
