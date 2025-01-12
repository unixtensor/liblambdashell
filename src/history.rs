use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::{rc::{self}, session::{self, MapDisplay}, valid_pbuf::IsValid};

#[derive(Debug, Clone)]
pub struct History {
	file: Option<PathBuf>,
	history: Vec<String>,
}
impl History {
	pub fn init() -> Self {
		Self {
			history: Vec::new(),
			file: rc::config_dir().map(|mut history_pbuf| {
				history_pbuf.push(".history");
				history_pbuf.is_valid_file_or_create(b"");
				history_pbuf
			}),
		}
	}

	pub fn add(&mut self, command: &str) {
		match self.history.last() {
		    Some(last_cmd) => if last_cmd != command { self.history.push(command.to_owned()); },
		    None => self.history.push(command.to_owned()),
		};
	}

	pub fn write_to_file_fallible(&mut self) {
		if !self.history.is_empty() {
			if let Some(history_file) = &self.file {
				OpenOptions::new().append(true).open(history_file.as_path()).map_or_display(|mut file| {
					let history_content = match self.history.len()==1 {
						true => format!("\n{}", self.history[0]),
						false => self.history.join("\n")
					};
					if let Err(write_err) = file.write_all(history_content.as_bytes()) {
						session::shell_error(write_err);
					};
				});
			}
		}
	}

	pub fn read_file_fallible(&self) -> Option<Vec<String>> {
		self.file.as_ref().and_then(|history_file| {
			File::open(history_file).map_or_display_none(|file| {
				Some(BufReader::new(file).lines().map_while(Result::ok).collect::<Vec<String>>())
			})
		})
	}
}