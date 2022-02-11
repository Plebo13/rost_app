use rost_app::Coin;

#[test]
fn test_invalid_isin() {
    assert_eq!(
        rost_app::get_etf_price(String::from("invalid_isin")).unwrap_err(),
        "Error response!"
    );
}

#[test]
fn test_get_coin_price() {
    rost_app::get_coin_price(&Coin::Bitcoin).unwrap();
}
