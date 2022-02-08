use scraper::Html;
use scraper::Selector;

const COIN_GECKO_URL: String =
    String::from("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur");
const LS_URL: String = String::from("https://www.ls-tc.de/de/etf/{}");

enum Coin {
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

pub fn get_coin_price(coin: Coin) -> Result<f32, &str> {
    let mut url = String::from();
}

pub fn get_etf_price(isin: &str) -> Result<f32, &str> {
    let url = format!(&LS_URL, isin);
    let http_response = reqwest::blocking::get(&url);
    let http_response = match http_response {
        Ok(http_response) => http_response,
        Err(_error) => return Err("Error during HTTP request!"),
    };

    if http_response.status().is_success() {
        let body = http_response.text();
        let body = match body {
            Ok(body) => body,
            Err(_error) => return Err("Error parsing the HTML document!"),
        };

        let fragment: scraper::Html = Html::parse_fragment(&body);
        let price = parse_asset_price(&fragment);
        match price {
            Some(price) => Ok(price),
            None => Err("Error parsing the HTML document!"),
        }
    } else {
        Err("Error response")
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
