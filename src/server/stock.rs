use dioxus::prelude::*;
use crate::models::StockQuote;
use anyhow::Result;
use reqwest::Error;
use futures::future::try_join_all;

#[post("/api/stock/quote")]
pub async fn get_stock_quote(symbol: String) -> Result<StockQuote, ServerFnError> {

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

#[post("/api/stocks/quotes")]
pub async fn get_multiple_stock_quotes(symbols: Vec<String>) -> Result<Vec<(String, StockQuote)>, ServerFnError> {
    dotenvy::dotenv().ok();
    
    let api_key = std::env::var("FINNHUB_API_KEY")
        .map_err(|_| ServerFnError::new("FINNHUB_API_KEY not found in environment"))?;
    
    eprintln!("[STOCKS] Fetching quotes for {} symbols (Finnhub free tier: 60 calls/minute)", symbols.len());
    
    // Create a single HTTP client with timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ServerFnError::new(format!("Failed to create HTTP client: {}", e)))?;
    
    // Process sequentially to respect rate limits (60 calls/minute = 1 call/second max)
    // This ensures we stay well under the limit
    let mut results = Vec::new();
    
    for (index, symbol) in symbols.iter().enumerate() {
        eprintln!("[STOCKS] Fetching quote {}/{} for {}", index + 1, symbols.len(), symbol);
        
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );
        
        let response = client.get(&url).send().await
            .map_err(|e| {
                eprintln!("[STOCKS] ERROR fetching {}: {}", symbol, e);
                ServerFnError::new(format!("Failed to fetch {}: {}", symbol, e))
            })?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            eprintln!("[STOCKS] ERROR API error for {}: {} - {}", symbol, status, text);
            return Err(ServerFnError::new(format!("API error for {}: {} - {}", symbol, status, text)));
        }
        
        let text = response.text().await
            .map_err(|e| {
                eprintln!("[STOCKS] ERROR reading response for {}: {}", symbol, e);
                ServerFnError::new(format!("Failed to read response for {}: {}", symbol, e))
            })?;

        let quote: StockQuote = serde_json::from_str(&text)
            .map_err(|e| {
                eprintln!("[STOCKS] ERROR parsing JSON for {}: {}. Response: {}", symbol, e, text);
                ServerFnError::new(format!("Failed to parse JSON for {}. Response: {}. Error: {}", symbol, text, e))
            })?;

        eprintln!("[STOCKS] SUCCESS fetched quote for {}", symbol);
        results.push((symbol.clone(), quote));
        
        // Non-blocking delay to respect rate limits (60/minute = ~1/second)
        if index < symbols.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        }
    }
    
    eprintln!("[STOCKS] SUCCESS fetched {} quotes total", results.len());
    Ok(results)
}
