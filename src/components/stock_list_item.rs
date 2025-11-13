use dioxus::prelude::*;
use crate::models::get_stock_name;
use crate::Route;

#[component]
pub fn StockListItem(
    symbol: String,
    selected: bool,
    on_click: EventHandler<String>,
) -> Element {
    let stock_name = get_stock_name(&symbol).unwrap_or_else(|| symbol.clone());
    let symbol_for_click = symbol.clone();
    
    rsx! {
        Link {
            to: Route::Stocks {
                symbol: symbol.clone(),
            },
            class: if selected { "selected" } else { "" },
            onclick: move |_| {
                eprintln!("[STOCK_LIST_ITEM] Clicked on symbol: {}", symbol_for_click);
                on_click.call(symbol_for_click.clone());
            },
            div { class: "symbol", "{symbol} {stock_name}" }
        }
    }
}

