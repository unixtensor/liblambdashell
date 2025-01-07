use std::{fs, io::{self}};

use crate::{
	vm::{LuauVm, self},
	commands,
	ps,
	rc,
	MapDisplay,
};

pub struct Config {
	pub norc: bool
}

pub struct LambdaShell {
	vm: LuauVm,
	ps1: String,
	config: Config,
	terminating: bool,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		Self {
			vm: vm::LuauVm::new(),
			ps1: ps::DEFAULT_PS.to_owned(),
			terminating: false,
			config,
		}
	}

	pub fn wait(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut io::stdout()).map(|()| {
			let mut input = String::new();
			io::stdin().read_line(&mut input).map_or_display(|_size| {
				match input.trim() {
					//special casey
					"exit" => self.terminating = true,
					trim => commands::Command::new(trim.to_owned()).exec()
				};
			})
		})
	}

	pub fn vm_exec(&self, source: String) {
		self.vm.exec(source);
	}

	pub fn start(&mut self) {
		if !self.config.norc {
			if let Some(conf_file) = rc::config_file() {
				fs::read_to_string(conf_file).map_or_display(|luau_conf| self.vm_exec(luau_conf));
			}
		}
		loop {
			match self.terminating {
				true => break,
				false => {

					match self.wait() {
						Ok(()) => {},
						Err(flush_error) => {
							println!("{flush_error}");
							break;
						}
					}
				},
			}
		}
	}
}
