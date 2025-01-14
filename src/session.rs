use std::{fs, io::{self}, sync::{Arc, Mutex}, thread};
use core::fmt;
use color_print::ceprintln;

use crate::{commands, history::History, ps::{self, Ps}, rc::{self}, vm::{self, LuauVm}};

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

trait Errors {
	fn error<E: fmt::Display>(&mut self, err: E);
}
impl Errors for LambdaShell {
	fn error<E: fmt::Display>(&mut self, err: E) {
		shell_error(err);
		self.terminate = true;
	}
}

trait Signals {
	fn sigterm_event(&mut self);
	fn input(&mut self);
}
impl Signals for LambdaShell {
	fn sigterm_event(&mut self) {
		// task::spawn(async move {
		// 	signal::ctrl_c().await.expect("Failed to listen for a sigterm signal.");
		// 	// self.history.write_to_file_fallible();
		// 	std::process::exit(0x0100);
		// });
	}
	fn input(&mut self) {
		thread::spawn(|| {
			loop {
				if self.terminate { break } else {
					match self.wait() {
				        Ok(()) => self.ps.borrow().display(),
				        Err(flush_err) => self.error(flush_err),
				    }
				}
			}
		});
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
	ps: Arc<Mutex<Ps>>,
}
impl LambdaShell {
	pub fn create(config: Config) -> Self {
		let ps = Arc::new(Mutex::new(Ps::set(ps::DEFAULT_PS.to_owned())));
		Self {
			ps: Arc::clone(&ps),
			vm: vm::LuauVm::new(ps),
			history: History::init(),
			terminate: false,
			config,
		}
	}

	fn wait(&mut self) -> Result<(), io::Error> {
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

	pub async fn start(&mut self) {
		if !self.config.norc {
			if let Some(conf_file) = rc::config_file() {
				fs::read_to_string(conf_file).map_or_display(|luau_conf| self.vm.exec(luau_conf));
			}
		};
		self.ps.borrow().display();
		self.sigterm_event();
		self.input();
	}
}