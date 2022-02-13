use rost_app::Coin;

#[test]
fn test_invalid_isin() {
    assert_eq!(
        rost_app::get_etf_price(String::from("invalid_isin")).unwrap_err(),
        "Error response!"
    );
}

#[test]
fn test_get_etf_price() {
    match rost_app::get_etf_price(String::from("LU1781541179")) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_bitcoin() {
    match rost_app::get_coin_price(&Coin::Bitcoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_bitcoin_string() {
    match rost_app::get_coin("bitcoin") {
        Some(_coin) => assert!(matches!(&Coin::Bitcoin, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_ethereum() {
    match rost_app::get_coin_price(&Coin::Ethereum) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_ethereum_string() {
    match rost_app::get_coin("ethereum") {
        Some(_coin) => assert!(matches!(&Coin::Ethereum, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_binance_coin() {
    match rost_app::get_coin_price(&Coin::BinanceCoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_binance_coin_string() {
    match rost_app::get_coin("binancecoin") {
        Some(_coin) => assert!(matches!(&Coin::BinanceCoin, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_tether() {
    match rost_app::get_coin_price(&Coin::Tether) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_tether_string() {
    match rost_app::get_coin("tether") {
        Some(_coin) => assert!(matches!(&Coin::Tether, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_solana() {
    match rost_app::get_coin_price(&Coin::Solana) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_solana_string() {
    match rost_app::get_coin("solana") {
        Some(_coin) => assert!(matches!(&Coin::Solana, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_cardano() {
    match rost_app::get_coin_price(&Coin::Cardano) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_cardano_string() {
    match rost_app::get_coin("cardano") {
        Some(_coin) => assert!(matches!(&Coin::Cardano, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_ripple() {
    match rost_app::get_coin_price(&Coin::Ripple) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_ripple_string() {
    match rost_app::get_coin("ripple") {
        Some(_coin) => assert!(matches!(&Coin::Ripple, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_usd_coin() {
    match rost_app::get_coin_price(&Coin::USDCoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_usd_coin_string() {
    match rost_app::get_coin("usd-coin") {
        Some(_coin) => assert!(matches!(&Coin::USDCoin, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_polkadot() {
    match rost_app::get_coin_price(&Coin::Polkadot) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_polkadot_string() {
    match rost_app::get_coin("polkadot") {
        Some(_coin) => assert!(matches!(&Coin::Polkadot, _coin)),
        None => panic!("No string returned!"),
    }
}

#[test]
fn test_get_dogecoin() {
    match rost_app::get_coin_price(&Coin::Dogecoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_dogecoin_string() {
    match rost_app::get_coin("dogecoin") {
        Some(_coin) => assert!(matches!(&Coin::Dogecoin, _coin)),
        None => panic!("No string returned!"),
    }
}
