pub mod profile;
pub use profile::Profile;

pub mod stock;
pub use stock::{StockQuote, StockCandle, StockSymbol, get_popular_stock_symbols, get_popular_stocks, get_stock_name};