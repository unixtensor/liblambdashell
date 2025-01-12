use std::{cell::RefCell, fs, io::{self}, rc::Rc};
use core::fmt;
use color_print::ceprintln;

use crate::{
	commands, history::History, ps::{self, Ps}, rc::{self}, vm::{self, LuauVm}
};

#[inline]
pub fn shell_error<E: fmt::Display>(err: E) {
	ceprintln!("<bold,r>[!]:</> {err}")
}

pub trait MapDisplay<T, E: fmt::Display> {
	fn map_or_display<F: FnOnce(T)>(self, f: F);
	fn map_or_display_none<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R>;
}
impl<T, E: fmt::Display> MapDisplay<T, E> for Result<T, E> {
	///Map but display the error to stdout
	#[inline]
	fn map_or_display<F: FnOnce(T)>(self, f: F) {
		self.map_or_else(|e| shell_error(e), f)
	}
	///Map but display the error to stdout and return `None`
	#[inline]
	fn map_or_display_none<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R> {
		self.map_or_else(|e| { shell_error(e); None }, f)
	}
}

#[derive(Debug, Clone)]
pub struct Config {
	pub norc: bool
}
pub struct LambdaShell {
	terminate: bool,
	history: History,
	config: Config,
	vm: LuauVm,
	ps: Rc<RefCell<Ps>>,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		let ps = Rc::new(RefCell::new(Ps::set(ps::DEFAULT_PS.to_owned())));
		Self {
			ps: Rc::clone(&ps),
			vm: vm::LuauVm::new(ps),
			history: History::init(),
			terminate: false,
			config,
		}
	}

	pub fn wait(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut io::stdout()).map(|()| {
			let mut input = String::new();
			io::stdin().read_line(&mut input).map_or_display(|_size| match input.trim() {
				"exit" => {
					self.terminate = true;
					self.history.add("exit");
				},
				trim => commands::Command::new(trim.to_owned()).exec(&mut self.history)
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
		self.ps.borrow().display();

		loop {
			if self.terminate { break } else {
				match self.wait() {
			        Ok(()) => self.ps.borrow().display(),
			        Err(flush_err) => self.error(flush_err),
			    }
			}
		}
		self.history.write_to_file_fallible();
	}

	pub fn vm_exec(&self, source: String) {
		self.vm.exec(source);
	}
}
