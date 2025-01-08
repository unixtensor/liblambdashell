use std::{fs, io::{self}};
use core::fmt;

use crate::{
	commands, ps::{self, Ps}, rc, shell_error, vm::{self, LuauVm}, MapDisplay
};

#[derive(Debug, Clone)]
pub struct Config {
	pub norc: bool
}

pub struct LambdaShell {
	terminate: bool,
	config: Config,
	vm: LuauVm,
	ps: Ps,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		let ps = Ps::set(ps::DEFAULT_PS.to_owned());
		Self {
			vm: vm::LuauVm::new(ps.to_owned()),
			terminate: false,
			config,
			ps
		}
	}

	pub fn wait(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut io::stdout()).map(|()| {
			let mut input = String::new();
			io::stdin().read_line(&mut input).map_or_display(|_size| match input.trim() {
				"exit" => self.terminate = true,
				trim => commands::Command::new(trim.to_owned()).exec()
			})
		})
	}

	pub fn error<E: fmt::Display>(&mut self, err: E) {
		shell_error(err);
		self.terminate = true;
	}

	pub fn start(&mut self) {
		if !self.config.norc {
			if let Some(conf_file) = rc::config_file() {
				fs::read_to_string(conf_file).map_or_display(|luau_conf| self.vm_exec(luau_conf));
			}
		}
		self.ps.display();

		loop {
			if self.terminate { break } else {
				match self.wait() {
			        Ok(()) => self.ps.display(),
			        Err(flush_err) => self.error(flush_err),
			    }
			}
		}
	}

	pub fn vm_exec(&self, source: String) {
		self.vm.exec(source);
	}
}
