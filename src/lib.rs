use scraper::Html;
use scraper::Selector;

pub fn get_etf_price(isin: &str) -> Result<f32, &str> {
    let mut url = String::from("https://www.ls-tc.de/de/etf/");
    url.push_str(isin);
    let http_response = reqwest::blocking::get(&url);
    let http_response = match http_response {
        Ok(http_response) => http_response,
        Err(_error) => return Err("Error during HTTP request!"),
    };

    if http_response.status().is_success() {
        let body = http_response.text();
        let body = match body {
            Ok(body) => body,
            Err(_error) => return Err("Error during HTML parsing!"),
        };

        let fragment = Html::parse_fragment(&body);
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
        Ok(price)
    } else {
        Err("Error response")
    }
}
