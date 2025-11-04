use dioxus::prelude::*;
use crate::models::StockQuote;
use anyhow::Result;
use reqwest::Error;

#[post("/api/stock/quote")]
pub async fn get_stock_quote(symbol: String) -> Result<StockQuote, ServerFnError> {

    // .env load
    // dotenvy::dotenv().ok();

    let api_key = std::env::var("FINNHUB_API_KEY")
        .map_err(|_| ServerFnError::new("FINNHUB_API_KEY not found in environment"))?;
    
    let url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch: {}", e)))?;
    
     if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("API error {}: {}", status, text)));
    }
    
    let text = response.text().await
        .map_err(|e| ServerFnError::new(format!("Failed to read response: {}", e)))?;
    
    
    let quote: StockQuote = serde_json::from_str(&text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse JSON. Response was: {}. Error: {}", text, e)))?;
    
    Ok(quote)
}