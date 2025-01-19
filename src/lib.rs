pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod session;
pub mod commands;
pub mod history;
pub mod terminal;
pub mod rc;
pub mod vm;

mod valid_pbuf;