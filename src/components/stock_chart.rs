use dioxus::prelude::*;
use dioxus_charts::{BarChart, LineChart};

use crate::server::stock::get_stock_custom_bars;
use crate::models::AggregateBar;
use crate::models::Period;

use web_sys::console;
use serde_json;
use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike, Duration};

const STOCK_CHART_CSS: Asset = asset!("/assets/styling/stock-chart.css");

fn timestamp_to_date_string(timestamp_ms: i64) -> String {
    let timestamp_secs = timestamp_ms / 1000;
    let datetime = match Utc.timestamp_opt(timestamp_secs, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return format!("{}", timestamp_secs), // Fallback if conversion fails
    };

    format!("{:02}.{:02}.", datetime.day(), datetime.month())
}


fn extract_labels(bars: &[AggregateBar]) -> Vec<String> {
    bars.iter()
        .map(|bar| timestamp_to_date_string(bar.timestamp))
        .collect()
}

fn extract_close_prices(bars: &[AggregateBar]) -> Vec<f32> {
    bars.iter()
        .map(|bar| bar.close as f32)
        .collect()
}

#[component]
pub fn StockChart(symbol: String, period: Period) -> Element {

    // Log to server console
    eprintln!("[CHART VIEW] Displaying chart for {} with period {}", symbol, period);
    // Log to browser console
    console::log_1(&format!("[CHART VIEW] Displaying chart for {} with period {}", symbol, period).into());

    let symbol_clone = symbol.clone();
    let custom_bars = use_server_future(move || {
        get_stock_custom_bars(symbol_clone.clone(), Some(period.to_string()))

    })?;

    let symbol_for_logging = symbol.clone();
    use_effect(move || {
        
        let bars = custom_bars(); // dioxus tracks this as a dependency by reading it here
        match bars {
            Some(Ok(custom_bars)) => {
                // log to server console like this
                // eprintln!("[CHART VIEW] Successfully loaded {} custom bars for {}", custom_bars.results.len(), symbol_for_logging);
                // log to to browser console like this
                // if let Ok(json_str) = serde_json::to_string_pretty(&custom_bars) {
                //     console::log_1(&format!("[CHART VIEW] Custom bars data for {}:\n{}", symbol_for_logging, json_str).into());
                // }
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
        document::Link { rel: "stylesheet", href: STOCK_CHART_CSS }
        div { class: "stock-chart",
            match custom_bars() {
                Some(Ok(bars)) => {
                    // Extract data for the chart
                    let labels = extract_labels(&bars.results);
                    let close_prices = extract_close_prices(&bars.results);

                    rsx! {
                        div { "Loaded {bars.results.len()} bars" }
                        //     BarChart {
                        //         padding_top: 30,
                        //         padding_left: 70,
                        //         padding_right: 50,
                        //         padding_bottom: 30,
                        //         bar_width: "5%",
                        //         horizontal_bars: false,
                        //         label_interpolation: (|v| format!("${:.2}", v)) as fn(f32) -> String,
                        //         series: vec![close_prices],
                        //         labels,
                        //     }
                        // }

            
                        div { class: "stock-chart-line-wrapper",
                            LineChart {
                                show_grid: true,
                                show_grid_ticks: false,
                                line_width: 0.4,
                                dot_size: 3,
                                padding_top: 30,
                                padding_left: 70,
                                padding_right: 50,
                                padding_bottom: 30,
                                label_interpolation: (|v| format!("${:.2}", v)) as fn(f32) -> String,
                                series: vec![close_prices],
                                labels,
                            }
                        }
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
    }

}
