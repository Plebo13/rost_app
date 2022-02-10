#[test]
fn test_invalid_isin() {
    assert_eq!(
        rost_app::get_etf_price("invalid_isin").unwrap_err(),
        "Error response!"
    );
}
