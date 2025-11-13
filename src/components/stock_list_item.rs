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
    let navigator = use_navigator();
    let symbol_for_nav = symbol.clone();
    let symbol_for_click = symbol.clone();
    let is_selected = selected;
    
    rsx! {
        div {
            class: if selected { "selected" } else { "" },
            tabindex: if selected { "0" } else { "-1" },
            onclick: move |_| {
                if is_selected {
                    // Second click: navigate
                    navigator
                        // First click: select the item
                        .push(Route::Stocks {
                            symbol: symbol_for_nav.clone(),
                        });
                } else {
                    on_click.call(symbol_for_click.clone());
                }
            },
            div { class: "symbol", "{symbol} {stock_name}" }
        }
    }
}

