use std::{path::PathBuf, fs::{self, File}, io::{self, Write}};
use thiserror::Error;

use crate::MapDisplay;

pub const DEFAULT_CONFIG_CONTENT: &str = r#"--!strict

local cyan = TERMINAL.OUT.FOREGROUND.CYAN

local username = cyan(SHELL.SYSTEM.USERNAME)
local hostname = SHELL.SYSTEM.HOSTNAME

SHELL.PROMPT = `{username}@{hostname} λ `"#;

#[derive(Debug, Error)]
#[allow(dead_code)]
enum IsValidDirErr {
	#[error("Failed to see if a file exists: {0}")]
	TryExists(#[from] io::Error),
	#[error("Not a valid entry")]
	NotAnEntry,
	#[error("Directory missing")]
	Missing
}

#[allow(dead_code)]
enum CreateErr {
	TryExists(io::Error),
	Passable
}

#[allow(dead_code)]
trait IsValid {
	fn is_valid(&self, is_dir_or_file: bool) -> Result<PathBuf, IsValidDirErr>;
	fn is_valid_option(&self, is_dir_or_file: bool) -> Option<PathBuf>;
	fn is_valid_file_or_create(&self, default_file_bytes: &[u8]) -> Option<PathBuf>;
	fn is_valid_dir_or_create(&self) -> Option<PathBuf>;
	fn is_valid_or<F>(&self, is_content: bool, f: F) -> Option<PathBuf>
	where
		F: FnOnce() -> Option<PathBuf>;
}
impl IsValid for PathBuf {
	fn is_valid(&self, is_content: bool) -> Result<PathBuf, IsValidDirErr> {
		match self.try_exists() {
			Ok(true) => match is_content {
				true => Ok(self.to_path_buf()),
				false => Err(IsValidDirErr::NotAnEntry)
			},
			Ok(false) => Err(IsValidDirErr::Missing),
			Err(try_e) => Err(IsValidDirErr::TryExists(try_e))
		}
	}

	fn is_valid_or<F>(&self, is_content: bool, f: F) -> Option<PathBuf>
	where
		F: FnOnce() -> Option<PathBuf>
	{
		self.is_valid(is_content).map_err(|e| match e {
			IsValidDirErr::TryExists(try_e) => CreateErr::TryExists(try_e),
			IsValidDirErr::NotAnEntry | IsValidDirErr::Missing => CreateErr::Passable
		}).map_or_else(|e| match e {
			CreateErr::TryExists(_) => None,
			CreateErr::Passable => f()
		}, Some)
	}

	fn is_valid_dir_or_create(&self) -> Option<PathBuf> {
		self.is_valid_or(self.is_dir(), || fs::create_dir(self).map_or_display_none(|()| Some(self.to_path_buf())))
	}

	fn is_valid_file_or_create(&self, default_file_bytes: &[u8]) -> Option<PathBuf> {
		self.is_valid_or(self.is_file(), || {
			File::create(self).map_or_display_none(|mut file| {
				file.write_all(default_file_bytes).map_or_display_none(|()| Some(self.to_path_buf()))
			})
		})
	}

	fn is_valid_option(&self, is_dir_or_file: bool) -> Option<PathBuf> {
		self.is_valid(is_dir_or_file).ok()
	}
}

pub fn config_dir() -> Option<PathBuf> {
	let mut config = home::home_dir()?;
	config.push(".config");
	config.is_valid_dir_or_create()?;
	config.push("lambdashell");
	config.is_valid_dir_or_create()
}

pub fn config_file() -> Option<PathBuf> {
	let mut config_file = config_dir()?;
	config_file.push("config.luau");
	config_file.is_valid_file_or_create(DEFAULT_CONFIG_CONTENT.as_bytes())
}

// TODO: history.rs
pub fn history_file() -> Option<PathBuf> {
	let mut config_file = config_dir()?;
	config_file.push(".history");
	config_file.is_valid_file_or_create(b"")
}