use std::{io, fs, io::Write, path::PathBuf};
use thiserror::Error;

const DEFAULT_CONFIG_CONTENT: &str = r#"--!strict

local Username = Shell.system.username
local Hostname = Shell.system.hostname

Shell.prompt = `{Username}@{Hostname} λ `"#;

#[derive(Debug, Error)]
enum RcError {
    #[error("Folder is missing")]
    FolderMissing,
    #[error("Failed to check folder existence: {0}")]
    FolderTryExists(#[from] io::Error),
}

trait is_valid {
	fn try_exists_handle(&self) -> bool;
	fn is_valid(&self) -> Option<PathBuf>;
	fn is_valid_silent(&self) -> Option<PathBuf>;
	fn is_valid_or_create(&self) -> Option<PathBuf>;
}
impl is_valid for PathBuf {
	fn try_exists_handle(&self) -> bool {
		self.try_exists().map_or_else(|e| {RcError::FolderTryExists(e)}, |exists| match exists {
			true => todo!(),
			false => todo!()
		})
	}

	fn is_valid(&self) -> Option<PathBuf> {
		self.try_exists().map_or_else(|e| {
            println!("{}", RcError::FolderTryExists(e));
            None
        }, |exists| match exists {
			true => Some(self.to_path_buf()),
			false => {
                println!("{}", RcError::FolderMissing);
                None
            }
		})
	}

	fn is_valid_silent(&self) -> Option<PathBuf> {
		self.try_exists().ok().map_or(None, |exists| match exists {
		    true => Some(self.to_path_buf()),
		    false => None,
		})
	}

	fn is_valid_or_create(&self) -> Option<PathBuf> {
		self.is_valid().map_or_else(|| {
			let new_dir = fs::create_dir(self).map_err(|e| println!("{e}"));
			return None
		}, |p_buf| Some(p_buf))
	}
}

fn config_dir() -> Option<PathBuf> {
	let mut config = home::home_dir()?;
	config.push(".config");
	config.is_valid()?;
	config.push("lambdashell");
	config.is_valid()
}

fn config_file() -> Option<PathBuf> {
	let mut config_file = config_dir()?;
	config_file.push("config.luau");

	if let Some(file) = config_file.is_valid_silent() {
		match file.is_file() {
			true => {

			},
			false => println!("{:?} is either not a file or permission was denied.", file.as_path().display())
		}
	}
	None
}