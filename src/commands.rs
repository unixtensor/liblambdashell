use uzers::User;
use std::{
	io,
	process,
	str::SplitWhitespace,
	path::{Path, PathBuf},
};

use crate::MapDisplay;

enum ValidStatus {
	NoRootFolder,
	TryExists(io::Error)
}

trait PathBufIsValid {
	fn is_valid(&self) -> Result<PathBuf, ValidStatus>;
	fn is_valid_or_home(&self) -> Option<PathBuf>;
}

trait ChangeDirectory {
	fn change_directory(&self, args: SplitWhitespace) -> Option<PathBuf>;
	fn set_current_dir(&self, new_path: &Path) -> Option<PathBuf>;
	fn specific_user_dir(&self, user: String) -> Option<PathBuf>;
	fn cd_args(&self, vec_args: Vec<String>) -> Option<PathBuf>;
	fn previous_dir(&self) -> Option<PathBuf>;
	fn home_dir(&self) -> Option<PathBuf>;
}

impl PathBufIsValid for PathBuf {
	fn is_valid(&self) -> Result<PathBuf, ValidStatus> {
		match self.try_exists() {
			Ok(true) => Ok(self.to_path_buf()),
			Ok(false) => Err(ValidStatus::NoRootFolder),
			Err(trye_error) => Err(ValidStatus::TryExists(trye_error))
		}
	}

	fn is_valid_or_home(&self) -> Option<PathBuf> {
		match self.is_valid() {
			Ok(valid) => Some(valid),
			Err(valid_status) => {
				match valid_status {
					ValidStatus::NoRootFolder => println!("cd: /root: No such file or directory"),
					ValidStatus::TryExists(error) => println!("cd: {error}"),
				};
				None
			},
		}
	}
}

impl ChangeDirectory for Command {
	fn set_current_dir(&self, new_path: &Path) -> Option<PathBuf> {
		std::env::set_current_dir(new_path).map_or_display_none(|()| Some(new_path.to_path_buf()))
	}

	fn home_dir(&self) -> Option<PathBuf> {
		home::home_dir().map_or(self.set_current_dir(Path::new("/")), |home_pathbuf| self.set_current_dir(&home_pathbuf))
	}

	fn previous_dir(&self) -> Option<PathBuf> {
		unimplemented!()
	}

	fn specific_user_dir(&self, requested_user: String) -> Option<PathBuf> {
		match requested_user.as_str() {
			"root" => PathBuf::from("/root").is_valid_or_home(),
			_ => {
				for user in unsafe { uzers::all_users().collect::<Vec<User>>() } {
					let user_name = user.name();
					if *requested_user == *user_name {
						let mut user_dir = PathBuf::from("/home");
						user_dir.push(user_name);
						return user_dir.is_valid_or_home();
					}
				}
				None
			}
		}
	}

	fn cd_args(&self, vec_args: Vec<String>) -> Option<PathBuf> {
		let string_path = vec_args.concat();
		let new_path = Path::new(string_path.as_str());
		match new_path.is_dir() {
			true => self.set_current_dir(new_path),
			false => {
				match new_path.file_name() {
					Some(file_name) => println!("cd: {:?} is not a directory.", file_name),
					None => println!("cd: Failed to resolve the file name of a file that is not a directory."),
				}
				None
			}
		}
	}

	fn change_directory(&self, args: SplitWhitespace) -> Option<PathBuf> {
		let vec_args: Vec<String> = args.map(|arg| arg.to_owned()).collect();
		match vec_args.first() {
			None => self.home_dir(),
			Some(arg) => match arg.as_str() {
				"/" => self.set_current_dir(Path::new("/")),
				"-" => self.previous_dir(),
				_ => {
					let mut arg_chars = arg.chars();
					match arg_chars.next() {
						Some(char) => match char == '~' {
							true => self.specific_user_dir(arg_chars.collect::<String>()),
							false => self.cd_args(vec_args),
						},
						None => self.home_dir(),
					}
				}
			},
		}
	}
}

pub struct Command(String);
impl Command {
	pub fn new(input: String) -> Self {
		Self(input)
	}

	pub fn spawn(&self, command_process: io::Result<process::Child>) {
		command_process.map_or_display_none(|mut child| Some(child.wait()));
	}

	pub fn exec(&self) {
		let mut args = self.0.split_whitespace();
		if let Some(command) = args.next() {
			match command {
				"cd" => {
					self.change_directory(args);
				},
				command => {
					self.spawn(process::Command::new(command).args(args).spawn());
				}
			}
		}
	}
}
