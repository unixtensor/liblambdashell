use crate::{ps, commands, rc, vm};
use std::{fs, io::{self}};

pub struct Config {
	pub norc: bool
}

struct Storage {
	pub command_exit_status: commands::ProcessExitStatus,
	pub ps1: String,
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
	terminating: bool,
	storage: Storage,
	config: Config,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		Self {
			storage: Storage {
				command_exit_status: None,
				ps1: ps::DEFAULT_PS.to_owned(),
			},
			terminating: false,
			config,
		}
	}

	pub fn wait(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut io::stdout()).map_or_else(|flush_error| Err(flush_error), |()| {
			let mut input = String::new();
			io::stdin().read_line(&mut input).map_or_else(|read_error| println!("{read_error}"), |_size| {
				let trimmed_input = input.trim();
				match trimmed_input {
					//special casey
					"exit" => self.terminating = true,
					_ => self.storage.command_exit_status = commands::Command::new(trimmed_input.to_owned()).exec()
				};
			});
			Ok(())
		})
	}

	fn rc_parse(&self) {
		if !self.config.norc {
			rc::config_file().map(|conf_file| fs::read_to_string(conf_file).map_or_else(
				|read_err|  println!("{read_err}"),
				|luau_conf| self.shell_vm_exec(luau_conf)
			));
		}
	}

	pub fn start(&mut self) {
		self.rc_parse();

		ps::display(&self.storage.ps1);

		loop {
			match self.terminating {
				true => break,
				false => match self.wait() {
					Ok(()) => ps::display(&self.storage.ps1),
					Err(flush_error) => {
						println!("{flush_error}");
						break;
					}
				},
			}
		}
	}
}
