# Rost-APP
[![DeepSource](https://deepsource.io/gh/Plebo13/rost_app.svg/?label=active+issues&show_trend=true&token=QEgtncVp7wyMh4OgysGxoVb7)](https://deepsource.io/gh/Plebo13/rost_app/?ref=repository-badge)

Rost-APP (Rost-AssetPriceProvider) is a small Rust library that lets you receive current prices for ETFs, stocks or cryptocurrnecies in EUR.

## Usage

To use SharePriceProvider simply import it into your python project. There are two main functions
available:

-   get_etf_price: Returns the current price of an ETF
-   get_coin_price: Returns the current price of a cryptocurrency

Supported coins:

-   Bitcoin
-   Ethereum
-   Binance Coin
-   Tether
-   Solana
-   Cardano
-   Ripple
-   USD Coin
-   Polkadot
-   Dogecoin

### Example

```rust
use rost_app::Coin;

let etf_price: f32 = rost_app::get_etf_price(&String::from("LU1781541179")).unwrap();
let coin_price: f32 = rost_app::get_coin_price(&Coin::Bitcoin).unwrap();
```

The above example prints the current prices of the *Lyxor Core MSCI World ETF*
and the current price of Bitcoin.
