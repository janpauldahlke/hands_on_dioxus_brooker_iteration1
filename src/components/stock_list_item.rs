use dioxus::prelude::*;
use crate::components::StockCard;
use crate::models::StockQuote;

#[component]
pub fn StockListItem(
    symbol: String,
    quote: StockQuote,
    selected: bool,
    on_click: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: if selected { "selected" } else { "" },
            onclick: move |_| on_click.call(symbol.clone()),
            StockCard { symbol: symbol.clone(), quote }
        }
    }
}

