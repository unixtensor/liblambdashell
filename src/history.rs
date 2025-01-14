use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::{rc::{self}, session::{self, MapDisplay}, valid_pbuf::IsValid};

#[derive(Debug, Clone)]
pub struct History {
	fs_history: Option<Vec<String>>,
	history: Vec<String>,
	file: Option<PathBuf>,
}
impl History {
	pub fn init() -> Self {
		let file = rc::config_dir().map(|mut config_dir| {
			config_dir.push(".history");
			config_dir.is_valid_file_or_create(b"");
			config_dir
		});
		let fs_history = file.as_ref().and_then(|file| {
			File::open(file).map_or_display_none(|file| {
				Some(BufReader::new(file).lines().map_while(Result::ok).collect::<Vec<String>>())
			})
		});
		Self {
			history: Vec::new(),
			fs_history,
			file,
		}
	}

	pub fn write_to_file_fallible(&mut self) {
		if self.history.is_empty() { return; }

		if let (Some(history_file), Some(fs_history)) = (&self.file, &self.fs_history) {
			OpenOptions::new()
				.append(true)
				.open(history_file.as_path())
			.map_or_display(|mut file| {
				let newline_maybe = if fs_history.is_empty() { "" } else { "\n" };
				let formatted = format!("{newline_maybe}{}", self.history.join("\n"));
				file.write_all(formatted.as_bytes()).unwrap_or_else(session::shell_error)
			});
		}
	}

	pub fn add(&mut self, command: &str) {
		match self.history.last() {
		    Some(last_cmd) => if last_cmd != command { self.history.push(command.to_owned()); },
		    None => self.history.push(command.to_owned()),
		};
	}
}