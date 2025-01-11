use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::{rc::{self}, shell_error, valid_pbuf::IsValid, MapDisplay};

pub struct History {
	history_file: PathBuf,
	checked_empty: bool
}
impl History {
	pub fn init() -> Option<Self> {
		rc::config_dir().map(|mut config| {
			config.push(".history");
			config.is_valid_file_or_create(b"");
			Self {
				history_file: config,
				checked_empty: false
			}
		})
	}

	pub fn is_empty(&mut self) -> bool {
		match self.checked_empty {
			true => true,
			false => self.read().map_or(false, |history_l| {
				self.checked_empty = true;
				history_l.is_empty()
			})
		}
	}

	pub fn write<S: AsRef<str>>(&mut self, content: S) {
		OpenOptions::new().append(true).open(self.history_file.as_path()).map_or_display(|mut file| {
			let write_data = match self.is_empty() {
			    true => content.as_ref().to_owned(),
			    false => format!("\n{}", content.as_ref()),
			};
			if let Err(write_err) = file.write_all(write_data.as_bytes()) {
				shell_error(write_err);
			};
		});
	}

	pub fn read(&self) -> Option<Vec<String>> {
		File::open(&self.history_file).map_or_display_none(|file| {
			Some(BufReader::new(file).lines().map_while(Result::ok).collect::<Vec<String>>())
		})
	}
}