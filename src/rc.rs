use std::path::PathBuf;

use crate::valid_pbuf::IsValid;

pub const DEFAULT_CONFIG_CONTENT: &str = r#"--!strict

local cyan = TERMINAL.OUT.FOREGROUND.CYAN
local red  = TERMINAL.OUT.FOREGROUND.RED

local hostname = SHELL.SYSTEM.HOSTNAME
local username = SHELL.SYSTEM.USERNAME

username = if username == "root" then red(username) else cyan(username)

SHELL.PROMPT = `{username}@{hostname} λ `"#;

pub fn config_dir() -> Option<PathBuf> {
	let mut config = home::home_dir()?;
	config.push(".config");
	config.is_valid_dir_or_create()?;
	config.push("lambdashell");
	config.is_valid_dir_or_create()
}

pub fn config_file() -> Option<PathBuf> {
	let mut config_file = config_dir()?;
	config_file.push("init.luau");
	config_file.is_valid_file_or_create(DEFAULT_CONFIG_CONTENT.as_bytes())
}