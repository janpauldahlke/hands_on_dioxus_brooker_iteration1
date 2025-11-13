use dioxus::prelude::*;
use crate::components::StockListItem;

#[component]
pub fn StocksList(
    symbols: Vec<String>,
    selected_symbol: Signal<Option<String>>,
    on_symbol_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "stocks-container",
            div { class: "stocks-list", id: "stocks-list", tabindex: "0",
                for symbol in symbols.iter() {
                    StockListItem {
                        symbol: symbol.clone(),
                        selected: selected_symbol() == Some(symbol.clone()),
                        on_click: move |sym| on_symbol_select.call(sym),
                    }
                }
            }
        }
    }
}

