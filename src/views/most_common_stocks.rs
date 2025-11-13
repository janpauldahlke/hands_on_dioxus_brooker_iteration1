use dioxus::prelude::*;
use crate::models::get_popular_stock_symbols;
use crate::components::StocksList;

#[component]
pub fn MostCommonStocks() -> Element {
    let mut selected_symbol = use_signal(|| None::<String>);
    let symbols = get_popular_stock_symbols();

    rsx! {
        div { id: "most-common-stocks",
            h1 { "Most Common Stocks" }

            StocksList {
                symbols,
                selected_symbol,
                on_symbol_select: move |sym| selected_symbol.set(Some(sym)),
            }
        }
    }
}
