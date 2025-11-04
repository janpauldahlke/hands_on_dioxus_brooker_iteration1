use dioxus::prelude::*;
use crate::models::StockQuote;

#[component]
pub fn StockCard(symbol: String, quote: StockQuote) -> Element {
    
    let change = quote.current_price - quote.previous_close;
    let change_percent = (change / quote.previous_close) * 100.0;
    
    rsx! {
        div { class: "stock-card",
            h2 { "{symbol}" }
            div { class: "price-info",
                p { class: "current-price", "Current: {quote.current_price:.2}" }
                p { class: if change >= 0.0 { "positive" } else { "negative" },
                    "Change: {change:.2} ({change_percent:.2}%)"
                }
            }
            div { class: "price-details",
                p { "High: {quote.high:.2}" }
                p { "Low: {quote.low:.2}" }
                p { "Open: {quote.open:.2}" }
                p { "Previous Close: {quote.previous_close:.2}" }
            }
        }
    }
}