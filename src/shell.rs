use crate::{ps, commands, rc, vm};
use std::{fs, io::{self}};

pub struct Config {
	pub norc: bool
}

trait ShellLuauVm {
	fn shell_vm_exec(&self, source: String);
}
impl ShellLuauVm for LambdaShell {
	fn shell_vm_exec(&self, source: String) {
		vm::Vm::new().exec(source);
	}
}

pub struct LambdaShell {
	ps1: String,
	config: Config,
	terminating: bool,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		Self {
			ps1: ps::DEFAULT_PS.to_owned(),
			terminating: false,
			config,
		}
	}

	pub fn wait(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut io::stdout()).map(|()| {
			let mut input = String::new();
			io::stdin().read_line(&mut input).map_or_else(|read_error| println!("{read_error}"), |_size| {
				match input.trim() {
					//special casey
					"exit" => self.terminating = true,
					trim => commands::Command::new(trim.to_owned()).exec()
				};
			})
		})
	}

	fn rc_parse(&self) {
		if !self.config.norc {
			if let Some(conf_file) = rc::config_file() {
				match fs::read_to_string(conf_file) {
			        Ok(luau_conf) => self.shell_vm_exec(luau_conf),
			        Err(read_err) => println!("{read_err}"),
			    }
			}
		}
	}

	pub fn start(&mut self) {
		self.rc_parse();

		ps::display(&self.ps1);

		loop {
			match self.terminating {
				true => break,
				false => match self.wait() {
					Ok(()) => ps::display(&self.ps1),
					Err(flush_error) => {
						println!("{flush_error}");
						break;
					}
				},
			}
		}
	}
}
