use dioxus::prelude::*;
use crate::models::get_popular_stock_symbols;
use crate::components::StocksList;

#[component]
pub fn MostCommonStocks() -> Element {
    let mut selected_symbol = use_signal(|| None::<String>);
    let symbols = get_popular_stock_symbols();
    let symbols_clone1 = symbols.clone();
    let symbols_clone2 = symbols.clone();
    
    // Initialize to first item if nothing is selected
    use_effect(move || {
        if selected_symbol().is_none() && !symbols_clone1.is_empty() {
            selected_symbol.set(Some(symbols_clone1[0].clone()));
        }
    });

    rsx! {
        div {
            id: "most-common-stocks",
            tabindex: "0",
            onkeydown: move |e| {
                let current_selected = selected_symbol();
                let current_index = current_selected
                    .and_then(|sym| symbols_clone2.iter().position(|s| s == &sym))
                    .unwrap_or(0);

                match e.key() {
                    Key::ArrowDown => {
                        let next_index = if current_index + 1 >= symbols_clone2.len() {
                            0 // Wrap to first
                        } else {
                            current_index + 1
                        };
                        selected_symbol.set(Some(symbols_clone2[next_index].clone()));
                        eprintln!(
                            "[MOST_COMMON_STOCKS] Arrow Down - Selected index: {}, symbol: {}",
                            next_index,
                            symbols_clone2[next_index],
                        );
                    }
                    Key::ArrowUp => {
                        let next_index = if current_index == 0 {
                            symbols_clone2.len() - 1
                        } else {
                            current_index - 1
                        };
                        selected_symbol.set(Some(symbols_clone2[next_index].clone()));
                        eprintln!(
                            "[MOST_COMMON_STOCKS] Arrow Up - Selected index: {}, symbol: {}",
                            next_index,
                            symbols_clone2[next_index],
                        );
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
