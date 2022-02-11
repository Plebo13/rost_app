use rost_app::Coin;

#[test]
fn test_invalid_isin() {
    assert_eq!(
        rost_app::get_etf_price(String::from("invalid_isin")).unwrap_err(),
        "Error response!"
    );
}

#[test]
fn test_get_bitcoin() {
    match rost_app::get_coin_price(&Coin::Bitcoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
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
fn test_get_binance_coin() {
    match rost_app::get_coin_price(&Coin::BinanceCoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
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
fn test_get_solana() {
    match rost_app::get_coin_price(&Coin::Solana) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
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
fn test_get_ripple() {
    match rost_app::get_coin_price(&Coin::Ripple) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
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
fn test_get_polkadot() {
    match rost_app::get_coin_price(&Coin::Polkadot) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}

#[test]
fn test_get_dogecoin() {
    match rost_app::get_coin_price(&Coin::Dogecoin) {
        Ok(_price) => assert!(true),
        Err(_error) => panic!("Unexpected exception!"),
    }
}
