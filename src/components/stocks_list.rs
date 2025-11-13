use dioxus::prelude::*;
use crate::components::StockListItem;
use crate::models::StockQuote;

#[component]
pub fn StocksList(
    stocks: Vec<(String, StockQuote)>,
    selected_symbol: Signal<Option<String>>,
    on_symbol_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "stocks-container",
            div { class: "stocks-list",
                for (symbol , quote) in stocks.iter() {
                    StockListItem {
                        symbol: symbol.clone(),
                        quote: quote.clone(),
                        selected: selected_symbol() == Some(symbol.clone()),
                        on_click: move |sym| on_symbol_select.call(sym),
                    }
                }
            }
        }
    }
}

