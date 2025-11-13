use dioxus::prelude::*;
use crate::models::get_popular_stock_symbols;
use crate::components::StocksList;
use crate::Route;

#[component]
pub fn MostCommonStocks() -> Element {
    let mut selected_symbol = use_signal(|| None::<String>);
    let symbols = get_popular_stock_symbols();
    let symbols_clone = symbols.clone();
    let navigator = use_navigator();

    rsx! {
        div {
            id: "most-common-stocks",
            tabindex: "0",
            onkeydown: move |e| {
                let current_selected = selected_symbol();
                let current_index = current_selected
                    .and_then(|sym| symbols_clone.iter().position(|s| s == &sym))
                    .unwrap_or(0);

                match e.key() {
                    Key::ArrowDown => {
                        let next_index = if current_index + 1 >= symbols_clone.len() {
                            0 // Wrap to first
                        } else {
                            current_index + 1
                        };
                        selected_symbol.set(Some(symbols_clone[next_index].clone()));

                    }
                    Key::ArrowUp => {
                        let next_index = if current_index == 0 {
                            symbols_clone.len() - 1
                        } else {
                            current_index - 1
                        };
                        selected_symbol.set(Some(symbols_clone[next_index].clone()));
                    }
                    Key::Enter => {
                        e.prevent_default();
                        if let Some(symbol) = selected_symbol() {
                            navigator
                                .push(Route::Stocks {
                                    symbol: symbol.clone(),
                                });
                        }
                    }
                    _ => {}
                }
            },
            h1 { "Most Common Stocks" }

            StocksList {
                symbols,
                selected_symbol,
                on_symbol_select: move |sym| {
                    eprintln!("[MOST_COMMON_STOCKS] Selected symbol: {}", sym);
                    selected_symbol.set(Some(sym));
                },
            }
        }
    }
}
