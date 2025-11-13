use dioxus::prelude::*;
use crate::server::stock::get_multiple_stock_quotes;
use crate::models::get_popular_stock_symbols;
use crate::components::StocksList;

#[component]
pub fn MostCommonStocks() -> Element {
    let mut selected_symbol = use_signal(|| None::<String>);
    let stocks_data = use_server_future(move || {
        let symbols = get_popular_stock_symbols();
        get_multiple_stock_quotes(symbols)
    });

    rsx! {
        div { id: "most-common-stocks",
            h1 { "Most Common Stocks" }

            if let Ok(resource) = stocks_data {
                if let Some(data) = resource() {
                    if let Ok(stocks) = data {
                        StocksList {
                            stocks,
                            selected_symbol,
                            on_symbol_select: move |sym| selected_symbol.set(Some(sym)),
                        }
                    } else {
                        div { "Failed to load stocks. Please check server logs." }
                    }
                } else {
                    div { "Loading stocks..." }
                }
            } else {
                div { "Failed to initialize stock data loading." }
            }
        }
    }
}
