use scraper::Html;
use scraper::Selector;

/// Enum representing all supported cryptocurrencies.
pub enum Coin {
    Bitcoin,
    Ethereum,
    BinanceCoin,
    Tether,
    Solana,
    Cardano,
    Ripple,
    USDCoin,
    Polkadot,
    Dogecoin,
}

/// Returns the coin matching the given string.
///
/// # Arguments
///
/// * `coin_string` - A string for a cryptocurrency
///
/// # Examples
///
/// ```
/// use rost_app::Coin;
///
/// let coin: &Coin = rost_app::get_coin("bitcoin").unwrap();
/// ```
pub fn get_coin(coin_string: &str) -> Option<&Coin> {
    match coin_string {
        "bitcoin" => Some(&Coin::Bitcoin),
        "ethereum" => Some(&Coin::Ethereum),
        "binancecoin" => Some(&Coin::BinanceCoin),
        "tether" => Some(&Coin::Tether),
        "solana" => Some(&Coin::Solana),
        "cardano" => Some(&Coin::Cardano),
        "ripple" => Some(&Coin::Ripple),
        "usd-coin" => Some(&Coin::USDCoin),
        "polkadot" => Some(&Coin::Polkadot),
        "dogecoin" => Some(&Coin::Dogecoin),
        _ => None,
    }
}

/// Returns the price for a given coin.
///
/// # Arguments
///
/// * `coin` - The given coin
///
/// # Examples
///
/// ```
/// use rost_app::Coin;
///
/// let price: f32 = rost_app::get_coin_price(&Coin::Bitcoin).unwrap();
/// ```
pub fn get_coin_price(coin: &Coin) -> Result<f32, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur",
        get_coin_string(coin)
    );
    let body = request(url)?;
    let first_split: Vec<&str> = body.split("\"eur\":").collect();
    let second_split: Vec<&str> = first_split[1].split('}').collect();
    let price: f32 = second_split[0].parse::<f32>().unwrap();
    Ok(price)
}

/// Returns the price for an ETF.
///
/// # Arguments
///
/// * `isin` - The ISIN of the ETF
///
/// # Examples
///
/// ```
/// let price: f32 = rost_app::get_etf_price(String::from("LU1781541179")).unwrap();
/// ```
pub fn get_etf_price(isin: &str) -> Result<f32, String> {
    let url = format!("https://www.ls-tc.de/de/etf/{}", isin);
    let body = request(url)?;
    let fragment: scraper::Html = Html::parse_fragment(&body);
    match parse_asset_price(&fragment) {
        Some(price) => Ok(price),
        None => Err(String::from("Error parsing the HTML document!")),
    }
}

fn get_coin_string(coin: &Coin) -> &str {
    match coin {
        Coin::Bitcoin => "bitcoin",
        Coin::Ethereum => "ethereum",
        Coin::BinanceCoin => "binancecoin",
        Coin::Tether => "tether",
        Coin::Solana => "solana",
        Coin::Cardano => "cardano",
        Coin::Ripple => "ripple",
        Coin::USDCoin => "usd-coin",
        Coin::Polkadot => "polkadot",
        Coin::Dogecoin => "dogecoin",
    }
}

fn request(url: String) -> Result<String, String> {
    let http_response = match reqwest::blocking::get(url) {
        Ok(http_response) => http_response,
        Err(_error) => return Err(String::from("Error during HTTP request!")),
    };

    if http_response.status().is_success() {
        match http_response.text() {
            Ok(body) => Ok(body),
            Err(_error) => Err(String::from("Error parsing the HTML document!")),
        }
    } else {
        Err(String::from("Error response!"))
    }
}

fn parse_asset_price(fragment: &scraper::Html) -> Option<f32> {
    let div_selector = Selector::parse("div.mono").unwrap();
    let span_selector = Selector::parse("span").unwrap();
    let parsed_price = fragment
        .select(&div_selector)
        .next()
        .unwrap()
        .select(&span_selector)
        .next()
        .unwrap()
        .inner_html();
    let price_string = parsed_price.replace(".", "").replace(",", ".");
    let price: f32 = price_string.parse::<f32>().unwrap();
    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coin_string() {
        assert_eq!("bitcoin", get_coin_string(&Coin::Bitcoin));
        assert_eq!("ethereum", get_coin_string(&Coin::Ethereum));
        assert_eq!("binancecoin", get_coin_string(&Coin::BinanceCoin));
        assert_eq!("tether", get_coin_string(&Coin::Tether));
        assert_eq!("solana", get_coin_string(&Coin::Solana));
        assert_eq!("cardano", get_coin_string(&Coin::Cardano));
        assert_eq!("ripple", get_coin_string(&Coin::Ripple));
        assert_eq!("usd-coin", get_coin_string(&Coin::USDCoin));
        assert_eq!("polkadot", get_coin_string(&Coin::Polkadot));
        assert_eq!("dogecoin", get_coin_string(&Coin::Dogecoin));
    }
}
