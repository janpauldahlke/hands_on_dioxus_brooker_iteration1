use serde::{Serialize, Deserialize};


// At the top, define a constant array of tuples
const POPULAR_STOCKS_DATA: &[(&str, &str)] = &[
    ("AAPL", "Apple Inc."),
    ("MSFT", "Microsoft Corporation"),
    ("AMZN", "Amazon.com Inc."),
    ("NVDA", "NVIDIA Corporation"),
    ("GOOGL", "Alphabet Inc. (Google)"),
    ("META", "Meta Platforms Inc. (Facebook)"),
    ("TSLA", "Tesla Inc."),
    ("BRK.B", "Berkshire Hathaway Inc."),
    ("V", "Visa Inc."),
    ("JNJ", "Johnson & Johnson"),
    ("WMT", "Walmart Inc."),
    ("JPM", "JPMorgan Chase & Co."),
    ("MA", "Mastercard Inc."),
    ("PG", "Procter & Gamble Co."),
    ("UNH", "UnitedHealth Group Inc."),
    ("HD", "The Home Depot Inc."),
    ("DIS", "The Walt Disney Company"),
    ("PYPL", "PayPal Holdings Inc."),
    ("BAC", "Bank of America Corp."),
    ("VZ", "Verizon Communications Inc."),
    ("ADBE", "Adobe Inc."),
    ("CMCSA", "Comcast Corporation"),
    ("NFLX", "Netflix Inc."),
    ("KO", "The Coca-Cola Company"),
    ("NKE", "Nike Inc."),
    ("MRK", "Merck & Co. Inc."),
    ("PFE", "Pfizer Inc."),
    ("T", "AT&T Inc."),
    ("INTC", "Intel Corporation"),
    ("CSCO", "Cisco Systems Inc."),
];

pub fn get_popular_stocks() -> Vec<StockSymbol> {
    POPULAR_STOCKS_DATA
        .iter()
        .map(|(symbol, name)| StockSymbol {
            symbol: symbol.to_string(),
            name: name.to_string(),
        })
        .collect()
}

pub fn get_popular_stock_symbols() -> Vec<String> {
    POPULAR_STOCKS_DATA
        .iter()
        .map(|(symbol, _)| symbol.to_string())
        .collect()
}

pub fn get_stock_name(symbol: &str) -> Option<String> {
    POPULAR_STOCKS_DATA
        .iter()
        .find(|(sym, _)| *sym == symbol)
        .map(|(_, name)| name.to_string())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockQuote {
    #[serde(rename = "c")]
    pub current_price: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "pc")]
    pub previous_close: f64,
    #[serde(rename = "t")]
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StockSymbol {
    pub symbol: String,
    pub name: String,
}
