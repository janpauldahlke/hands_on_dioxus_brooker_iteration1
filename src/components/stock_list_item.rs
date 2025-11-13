use dioxus::prelude::*;
use crate::models::get_stock_name;

#[component]
pub fn StockListItem(
    symbol: String,
    selected: bool,
    on_click: EventHandler<String>,
) -> Element {
    let stock_name = get_stock_name(&symbol).unwrap_or_else(|| symbol.clone());
    
    rsx! {
        div {
            class: if selected { "selected" } else { "" },
            onclick: move |_| on_click.call(symbol.clone()),
            div { class: "symbol", "{symbol} {stock_name}" }
        }
    }
}

