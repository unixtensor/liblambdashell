use std::{cell::RefCell, fs, rc::Rc};
use core::fmt;
use color_print::ceprintln;

use crate::{
	history::History, ps::{self, Ps}, rc::{self}, terminal, vm::LuauVm
};

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

pub fn shell_error<E: fmt::Display>(err: E) {
	ceprintln!("<bold,r>[!]:</> {err}")
}
pub fn shell_error_none<T, E: fmt::Display>(err: E) -> Option<T> {
	shell_error(err);
	None
}

#[derive(Debug, Clone)]
pub struct Config {
	pub norc: bool,
	pub nojit: bool,
	pub nosandbox: bool
}
pub struct LambdaShell {
	history: History,
	config: Config,
	ps: Ps,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		Self {
			ps: Ps::set(ps::DEFAULT_PS.to_owned()),
			history: History::init(),
			config,
		}
	}

	pub fn start(&mut self) {
		if !self.config.norc {
			if let Some(conf_file) = rc::config_file() {
				fs::read_to_string(conf_file).map_or_display(|luau_conf| self.vm_exec(luau_conf));
			}
		}
		terminal::Processor::init()
			.input_processor(&mut self.history)
			.map_or_display(|()| self.history.write_to_file_fallible());
	}

	pub fn vm_exec(&self, source: String) {
		let p = Rc::new(RefCell::new(&self.ps));
		LuauVm::new(Rc::clone(p)).exec(source);
	}
}
