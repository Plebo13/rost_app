use scraper::Html;
use scraper::Selector;

pub fn get_etf_price(isin: &str) -> Result<f32, reqwest::Error> {
    let mut url = String::from("https://www.ls-tc.de/de/etf/");
    url.push_str(isin);
    let body = reqwest::blocking::get(&url)?.text()?;
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
}
