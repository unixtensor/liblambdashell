use std::{io, path::PathBuf};

pub enum RcError {
	FolderMissing,
	FolderTryExists(io::Error)
}

trait is_valid {
	fn is_valid(&self) -> Option<PathBuf>;
}

impl is_valid for PathBuf {
	fn is_valid(&self) -> Option<PathBuf> {
		let try_exists = match self.try_exists() {
			Ok(config_exist) => match config_exist {
			    true => Ok(self.to_path_buf()),
			    false => Err(RcError::FolderMissing),
			},
		    Err(trye_error) => Err(RcError::FolderTryExists(trye_error)),
		};
		match try_exists {
		    Ok(file) => Some(file.to_path_buf()),
		    Err(rc_error) => match rc_error {
		        RcError::FolderMissing => todo!(),
		        RcError::FolderTryExists(error) => {
					println!("{error}");
					None
				},
		    },
		}
	}
}

fn dot_config_folder() -> Option<PathBuf> {
	let mut config = home::home_dir()?;
	config.push(".config");
	config.is_valid()
}

fn rc_folder() -> Option<PathBuf> {
	let mut dot_config = dot_config_folder()?;
	dot_config.push("lambdashell");
	dot_config.is_valid()
}
