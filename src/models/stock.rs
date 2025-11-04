use serde::{Serialize, Deserialize};

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