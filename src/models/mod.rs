pub mod profile;
pub use profile::Profile;

pub mod stock;
pub use stock::{StockQuote, StockSymbol, get_popular_stock_symbols, get_popular_stocks};