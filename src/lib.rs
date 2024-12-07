pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod shell;
mod commands;
mod ps;
mod rc;