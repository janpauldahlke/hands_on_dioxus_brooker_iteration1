use dioxus::prelude::*;
use dioxus_charts::BarChart;
use web_sys::console;
use crate::server::stock::get_stock_custom_bars;
use serde_json;

#[component]
pub fn StockChart(symbol: String) -> Element {

    let symbol_clone = symbol.clone();
    let custom_bars = use_server_future(move || {
        get_stock_custom_bars(symbol_clone.clone(), Some("MONTH".to_string()))

    })?;

    let symbol_for_logging = symbol.clone();
    use_effect(move || {
        
        let bars = custom_bars(); // dioxus tracks this as a dependency by reading it here
        match bars {
            Some(Ok(custom_bars)) => {
                // log to server console like this
                eprintln!("[CHART VIEW] Successfully loaded {} custom bars for {}", custom_bars.results.len(), symbol_for_logging);
                // log to to browser console like this
                if let Ok(json_str) = serde_json::to_string_pretty(&custom_bars) {
                    console::log_1(&format!("[CHART VIEW] Custom bars data for {}:\n{}", symbol_for_logging, json_str).into());
                }
            }
            Some(Err(e)) => {
                eprintln!("[CHART VIEW] Error loading custom bars for {}: {}", symbol_for_logging, e);
                console::log_1(&format!("[CHART VIEW] Error: {}", e).into());
            }
            None => {
                eprintln!("[CHART VIEW] Loading custom bars for {}...", symbol_for_logging);
                console::log_1(&format!("[CHART VIEW] Loading custom bars for {}...", symbol_for_logging).into());
            }
        }
    });

    rsx! {
        div { class: "stock-chart",
            match custom_bars() {
                Some(Ok(bars)) => {
                    rsx! {
                        div { "Loaded {bars.results.len()} bars" }
                    }
                }
                Some(Err(e)) => {
                    rsx! {
                        div { "Error: {e}" }
                    }
                }
                None => {
                    rsx! {
                        div { "Loading..." }
                    }
                }
            }
        }
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
