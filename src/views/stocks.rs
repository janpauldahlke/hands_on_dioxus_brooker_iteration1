use dioxus::prelude::*;
use crate::server::stock::{get_stock_quote, get_stock_candles};
use crate::components::StockCard;

#[component]
pub fn Stocks(symbol: String) -> Element {
    let symbol_clone = symbol.clone();
    let symbol_clone_for_candles = symbol.clone();
    let navigator = use_navigator();
    
    let stock_quote = use_server_future(move || {
        get_stock_quote(symbol_clone.clone()) 
    })?;
    
    // let stock_candles = use_server_future(move || {
    //     eprintln!("[STOCKS VIEW] Fetching candles for symbol: {}", symbol_clone_for_candles);
    //     get_stock_candles(symbol_clone_for_candles.clone(), Some("MONTH".to_string()))
    // })?;
    
    // Log when candles are loaded
    //let symbol_for_logging = symbol.clone();
    //use_effect(move || {
        // match stock_candles() {
        //     Some(Ok(candles)) => {
        //         eprintln!("[STOCKS VIEW] Successfully loaded {} candle data points for {}", candles.timestamps.len(), symbol_for_logging);
        //     }
        //     Some(Err(e)) => {
        //         eprintln!("[STOCKS VIEW] Error loading candles for {}: {}", symbol_for_logging, e);
        //     }
        //     None => {
        //         eprintln!("[STOCKS VIEW] Loading candles for {}...", symbol_for_logging);
        //     }
        // }
    //});

    
    rsx! {

        div {
            id: "stocks",
            tabindex: "0",
            onkeydown: move |e| {
                match e.key() {
                    Key::ArrowLeft => {
                        eprintln!("[STOCKS] ArrowLeft pressed - navigating back");
                        e.prevent_default();
                        navigator.go_back();
                    }
                    _ => {
                        eprintln!("[STOCKS] Key pressed: {:?}", e.key());
                    }
                }
            },
            h1 { "Stocks Overview" }
            button {
                onclick: move |_| {
                    navigator.go_back();
                },
                "← Back"
            }
            match stock_quote() {
                Some(Ok(quote)) => {
                    rsx! {
                        StockCard { symbol: symbol.clone(), quote }
                    }
                }
                Some(Err(e)) => {
                    rsx! {
                        div { "Error loading stock: {e}" }
                    }
                }
                None => {
                    rsx! {
                        div { "Loading stock data..." }
                    }
                }
            }
        }
    }
}