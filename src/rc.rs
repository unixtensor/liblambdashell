use std::{fs::{self, File}, io::{self, Write}, path::PathBuf};
use thiserror::Error;

const DEFAULT_CONFIG_CONTENT: &str = r#"--!strict

local username = SHELL.SYSTEM.USERNAME
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
		let possible_content = self.is_valid(is_content).map_err(|e| match e {
			IsValidDirErr::TryExists(try_e) => CreateErr::TryExists(try_e),
			IsValidDirErr::NotAnEntry | IsValidDirErr::Missing => CreateErr::Passable
		});
		match possible_content {
		    Ok(p) => Some(p),
		    Err(e) => match e {
			    CreateErr::TryExists(_) => None,
			    CreateErr::Passable => f()
			},
		}
	}

	fn is_valid_dir_or_create(&self) -> Option<PathBuf> {
		self.is_valid_or(self.is_dir(), || {
			match fs::create_dir(self) {
			    Ok(()) => Some(self.to_path_buf()),
			    Err(create_e) => {
					println!("{create_e}");
					None
				},
			}
		})
	}

	fn is_valid_file_or_create(&self, default_file_bytes: &[u8]) -> Option<PathBuf> {
		self.is_valid_or(self.is_file(), || {
			match File::create(self) {
			    Err(create_e) => {
					println!("{create_e}");
					None
				},
			    Ok(mut file) => match file.write_all(default_file_bytes) {
			        Ok(()) => Some(self.to_path_buf()),
			        Err(write_e) => {
						println!("{write_e}");
						None
					},
				},
			}
		})
	}

	fn is_valid_option(&self, is_dir_or_file: bool) -> Option<PathBuf> {
		self.is_valid(is_dir_or_file).map_or(None, |p| Some(p))
	}
}

pub fn config_dir() -> Option<PathBuf> {
	let mut config = home::home_dir()?;
	config.push(".config");
	config.is_valid_option(config.is_dir())?;
	config.push("lambdashell");
	config.is_valid_dir_or_create()
}

pub fn config_file() -> Option<PathBuf> {
	let mut config_file = config_dir()?;
	config_file.push("config.luau");
	config_file.is_valid_file_or_create(DEFAULT_CONFIG_CONTENT.as_bytes())
}

pub fn none() -> Option<PathBuf> {
	None
}