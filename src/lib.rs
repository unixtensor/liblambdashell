pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod session;
mod commands;
mod ps;
mod rc;

#[path = "./luau/vm.rs"]
mod vm;
#[path = "./luau/alias.rs"]
mod alias;
#[path = "./luau/terminal.rs"]
mod terminal;
#[path = "./luau/shell.rs"]
mod shell;