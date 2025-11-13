use dioxus::prelude::*;
use crate::server::stock::get_stock_quote;
use crate::components::StockCard;

#[component]
pub fn Stocks(symbol: String) -> Element {
    let symbol_clone = symbol.clone();
    
    let stock_quote = use_server_future(move || {
        get_stock_quote(symbol_clone.clone()) 
    })?;
    
    rsx! {
        div { id: "stocks",
            h1 { "Stocks Overview" }
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