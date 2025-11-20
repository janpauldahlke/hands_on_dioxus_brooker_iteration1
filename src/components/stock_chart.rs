use dioxus::prelude::*;
use dioxus_charts::BarChart;

use crate::server::stock::get_stock_custom_bars;

#[component]
pub fn StockChart(symbol: String) -> Element {

    let symbol_clone = symbol.clone();
    let custom_bars = use_server_future(move || {
        get_stock_custom_bars(symbol_clone.clone(), Some("MONTH".to_string()))

    })?;

    let symbol_for_logging = symbol.clone();
    use_effect(move || {
        match custom_bars() {
            Some(Ok(custom_bars)) => {
                eprintln!("[STOCKS VIEW] Successfully loaded {} custom bars for {}", custom_bars.results.len(), symbol_for_logging);
            }
            Some(Err(e)) => {
                eprintln!("[STOCKS VIEW] Error loading custom bars for {}: {}", symbol_for_logging, e);
            }
            None => {
                eprintln!("[STOCKS VIEW] Loading custom bars for {}...", symbol_for_logging);
            }
        }
    });

    rsx! {
        div { class: "stock-chart", "Chart placeholder for {symbol} - will add chart component here" }
        BarChart {
            padding_top: 30,
            padding_left: 70,
            padding_right: 50,
            padding_bottom: 30,
            bar_width: "10%",
            horizontal_bars: true,
            label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
            series: vec![vec![63.0, 14.4, 8.0, 5.1, 1.8]],
            labels: vec![
                "Chrome".into(),
                "Safari".into(),
                "IE/Edge".into(),
                "Firefox".into(),
                "Opera".into(),
            ],
        }
    }

}
