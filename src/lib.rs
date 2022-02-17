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
        "bitcoin" => return Some(&Coin::Bitcoin),
        "ethereum" => return Some(&Coin::Ethereum),
        "binancecoin" => return Some(&Coin::BinanceCoin),
        "tether" => return Some(&Coin::Tether),
        "solana" => return Some(&Coin::Solana),
        "cardano" => return Some(&Coin::Cardano),
        "ripple" => return Some(&Coin::Ripple),
        "usd-coin" => return Some(&Coin::USDCoin),
        "polkadot" => return Some(&Coin::Polkadot),
        "dogecoin" => return Some(&Coin::Dogecoin),
        _ => return None,
    };
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
    let second_split: Vec<&str> = first_split[1].split("}").collect();
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
/// let price: f32 = rost_app::get_etf_price(&String::from("LU1781541179")).unwrap();
/// ```
pub fn get_etf_price(isin: &String) -> Result<f32, String> {
    let url = format!("https://www.ls-tc.de/de/etf/{}", isin);
    let body = request(url)?;
    let fragment: scraper::Html = Html::parse_fragment(&body);
    match parse_asset_price(&fragment) {
        Some(price) => return Ok(price),
        None => return Err(String::from("Error parsing the HTML document!")),
    }
}

fn get_coin_string(coin: &Coin) -> &str {
    match coin {
        Coin::Bitcoin => return "bitcoin",
        Coin::Ethereum => return "ethereum",
        Coin::BinanceCoin => return "binancecoin",
        Coin::Tether => return "tether",
        Coin::Solana => return "solana",
        Coin::Cardano => return "cardano",
        Coin::Ripple => return "ripple",
        Coin::USDCoin => return "usd-coin",
        Coin::Polkadot => return "polkadot",
        Coin::Dogecoin => return "dogecoin",
    };
}

fn request(url: String) -> Result<String, String> {
    let http_response = match reqwest::blocking::get(url) {
        Ok(http_response) => http_response,
        Err(_error) => return Err(String::from("Error during HTTP request!")),
    };

    if http_response.status().is_success() {
        match http_response.text() {
            Ok(body) => return Ok(body),
            Err(_error) => return Err(String::from("Error parsing the HTML document!")),
        };
    } else {
        return Err(String::from("Error response!"));
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
    return Some(price);
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
