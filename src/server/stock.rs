use dioxus::prelude::*;
use crate::models::{StockQuote, StockCandle, CustomBarResponse, AggregateBar};
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

//  to use this pay 3000$ monthly for premiu, this endpoint is not availabe in the free tier
#[post("/api/stock/candles")]
pub async fn get_stock_candles(symbol: String, period: Option<String>) -> Result<StockCandle, ServerFnError> {
    dotenvy::dotenv().ok();
    
    let api_key = std::env::var("FINNHUB_API_KEY")
        .map_err(|_| ServerFnError::new("FINNHUB_API_KEY not found in environment"))?;
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| ServerFnError::new(format!("Failed to get current time: {}", e)))?
        .as_secs() as i64;
    
    // Determine resolution and time range based on period
    // TODO; moe this date validation into its own module
    let (resolution, from_timestamp) = match period.as_deref() {
        Some("MONTH") => {
            // Last 30 days with daily resolution
            ("D", now - (30 * 24 * 60 * 60))
        }
        Some("YEAR") => {
            // Last 365 days with daily resolution
            ("D", now - (365 * 24 * 60 * 60))
        }
        Some("OVERALL") => {
            // Last 5 years (capped at 5 years max) with monthly resolution
            let five_years_ago = now - (5 * 365 * 24 * 60 * 60);
            ("M", five_years_ago)
        }
        _ => {
            // Default to last 30 days with daily resolution
            ("D", now - (30 * 24 * 60 * 60))
        }
    };
    
    eprintln!("[STOCKS] Fetching candles for {} (period: {:?}, resolution: {}, from: {}, to: {})", symbol, period, resolution, from_timestamp, now);
    
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution={}&from={}&to={}&token={}",
        symbol, resolution, from_timestamp, now, api_key
    );
    
    let response = reqwest::get(&url).await
        .map_err(|e| {
            eprintln!("[STOCKS] ERROR fetching candles for {}: {}", symbol, e);
            ServerFnError::new(format!("Failed to fetch candles: {}", e))
        })?;
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        eprintln!("[STOCKS] ERROR API error for candles {}: {} - {}", symbol, status, text);
        return Err(ServerFnError::new(format!("API error {}: {}", status, text)));
    }
    
    let text = response.text().await
        .map_err(|e| {
            eprintln!("[STOCKS] ERROR reading response for candles {}: {}", symbol, e);
            ServerFnError::new(format!("Failed to read response: {}", e))
        })?;
    
    let candle: StockCandle = serde_json::from_str(&text)
        .map_err(|e| {
            eprintln!("[STOCKS] ERROR parsing JSON for candles {}: {}. Response: {}", symbol, e, text);
            ServerFnError::new(format!("Failed to parse JSON. Response: {}. Error: {}", text, e))
        })?;
    
    eprintln!("[STOCKS] SUCCESS fetched candles for {} ({} data points)", symbol, candle.timestamps.len());
    Ok(candle)
}


#[post("/api/stock/custom-bars")]
pub async fn get_stock_custom_bars(symbol: String, period: Option<String>) -> Result<CustomBarResponse, ServerFnError> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("MASSIVE_API_KEY")
        .map_err(|_| ServerFnError::new("MASSIVE_API_KEY not found in environment"))?;

        // example argument structure
        //https://api.massive.com/v2/aggs/ticker/AAPL/range/1/day/2023-01-09/2023-02-10?adjusted=true&sort=asc&limit=120&apiKey=TkwDmtA7dbomkd2yYL12qzTOopfXv1Dp
    
    let url = format!(
        "https://api.massive.com/v2/aggs/ticker/{}/range/1/day/2025-01-09/2025-02-10?adjusted=true&sort=asc&apiKey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch custom bars: {}", e)))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("API error {}: {}", status, text)));
    }
    
    let text = response.text().await
        .map_err(|e| ServerFnError::new(format!("Failed to read response: {}", e)))?;
    
    let custom_bar: CustomBarResponse = serde_json::from_str(&text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse JSON. Response was: {}. Error: {}", text, e)))?;
    
    Ok(custom_bar)
}