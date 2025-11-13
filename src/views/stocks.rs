use dioxus::prelude::*;
use crate::server::stock::get_stock_quote;
use crate::components::StockCard;

#[component]
pub fn Stocks(symbol: String) -> Element {
    let symbol_clone = symbol.clone();
    let navigator = use_navigator();
    
    let stock_quote = use_server_future(move || {
        get_stock_quote(symbol_clone.clone()) 
    })?;
    
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