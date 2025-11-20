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

// finnhub tech, not available in the free tier
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockCandle {
    #[serde(rename = "c")]
    pub close_prices: Vec<f64>,
    #[serde(rename = "h")]
    pub high_prices: Vec<f64>,
    #[serde(rename = "l")]
    pub low_prices: Vec<f64>,
    #[serde(rename = "o")]
    pub open_prices: Vec<f64>,
    #[serde(rename = "s")]
    pub status: String,
    #[serde(rename = "t")]
    pub timestamps: Vec<i64>,
    #[serde(rename = "v")]
    pub volumes: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StockSymbol {
    pub symbol: String,
    pub name: String,
}

// use this for stockchart data and use massive api key here
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomBarResponse {
    pub adjusted: bool,
    #[serde(rename = "next_url")]
    pub next_url: Option<String>,
    #[serde(rename = "queryCount")]
    pub query_count: i32,
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub results: Vec<AggregateBar>,
    #[serde(rename = "resultsCount")]
    pub results_count: i32,
    pub status: String,
    pub ticker: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateBar {
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "n")]
    pub number_of_transactions: Option<i32>,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "t")]
    pub timestamp: i64,  // Unix millisecond timestamp
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "vw")]
    pub volume_weighted_average_price: Option<f64>,
}