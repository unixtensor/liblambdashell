use uzers::User;
use std::{
    io,
    process,
    str::SplitWhitespace,
    path::{Path, PathBuf},
};

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
            Ok(root_folder_exist) => match root_folder_exist {
                true => Ok(self.to_path_buf()),
                false => Err(ValidStatus::NoRootFolder)
            },
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
	    match std::env::set_current_dir(new_path) {
	        Ok(()) => Some(new_path.to_path_buf()),
	        Err(set_cd_err) => {
				println!("{set_cd_err}");
				None
			},
	    }
	}

	fn home_dir(&self) -> Option<PathBuf> {
	    match home::home_dir() {
	        Some(home_path_buf) => self.set_current_dir(&home_path_buf),
	        None => self.set_current_dir(Path::new("/")),
	    }
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
        let vec_args: Vec<String> = args.map(|arg| arg.to_string()).collect();
        match vec_args.first() {
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
            None => self.home_dir(),
        }
    }
}

pub type ProcessExitStatus = Option<process::ExitStatus>;
pub struct Command(String);
impl Command {
    pub fn new(input: String) -> Self {
        Self(input)
    }

    pub fn spawn(&self, command_process: io::Result<process::Child>) -> ProcessExitStatus {
        match command_process {
            Ok(mut child) => Some(match child.wait() {
                Ok(exit_status) => exit_status,
                Err(exit_status_err) => {
                    println!("{exit_status_err}");
                    return None;
                }
            }),
            Err(e) => {
                println!("{e}");
                return None;
            }
        }
    }

    pub fn exec(&self) -> ProcessExitStatus {
        let mut args = self.0.split_whitespace();
        args.next().and_then(|command| match command {
            "cd" => {
                Self::change_directory(self, args);
                None
            }
            command => self.spawn(process::Command::new(command).args(args).spawn()),
        })
    }
}
