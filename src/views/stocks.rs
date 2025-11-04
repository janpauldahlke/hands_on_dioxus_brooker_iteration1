use dioxus::prelude::*;
use crate::server::stock::get_stock_quote;
use crate::components::StockCard;

#[component]
pub fn Stocks() -> Element {
    // US market apple stock hardcoded
    let stock_quote = use_server_future(move || get_stock_quote("AAPL".to_string()))?;
    
    rsx! {
        div { id: "stocks",
            h1 { "Stocks Overview" }
            match stock_quote() {
                Some(Ok(quote)) => {
                    rsx! {
                        StockCard { symbol: "AAPL".to_string(), quote }
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