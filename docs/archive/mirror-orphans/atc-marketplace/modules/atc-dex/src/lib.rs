//! Deterministic DEX core.
//! Monetary values are integer base units; floating-point arithmetic is forbidden.

pub mod fees;
pub mod matcher;
pub mod orderbook;
pub mod pool;
pub mod settlement;

pub use fees::FeeCalculator;
pub use matcher::OrderMatcher;
pub use orderbook::{Order, OrderBook, Side};
pub use pool::LiquidityPool;
pub use settlement::{Settlement, Trade};
