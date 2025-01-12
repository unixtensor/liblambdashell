use mlua::{Function, Lua as Luau, MultiValue, Result as lResult, Table, Value};
use color_print::{cformat, ceprintln};
use terminal::TerminalGlobal;
use std::{cell::RefCell, rc::Rc};
use core::fmt;
use shell::ShellGlobal;

use crate::{ps::Ps, session::MapDisplay};

mod shell;
mod terminal;
mod alias;

trait LuauRuntimeErr<T> {
	fn map_or_luau_rt_err<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R>;
}
impl<T, E: fmt::Display> LuauRuntimeErr<T> for Result<T, E> {
	#[inline]
	fn map_or_luau_rt_err<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R> {
		self.map_or_else(|luau_rt_err| {
			ceprintln!("<bold>====</>\n<r><bold>[!]:</> {luau_rt_err}</>\n<bold>====</>");
			None
		}, f)
	}
}

trait Globals {
	const LIB_VERSION: &str;
	const CONV_ERROR: &str;
	fn global_warn(&self, luau_globals: &Table) -> lResult<()>;
	fn global_version(&self, luau_globals: &Table) -> lResult<()>;
}
impl Globals for LuauVm {
	const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
	const CONV_ERROR: &str = "<SHELL CONVERSION ERROR>";

	fn global_warn(&self, luau_globals: &Table) -> lResult<()> {
		let luau_print = luau_globals.get::<Function>("print")?;
		luau_globals.raw_set("warn", self.vm.create_function(move |this, args: MultiValue| -> lResult<()> {
			let luau_multi_values = args.into_iter()
				.map(|value| cformat!("<bold,y>{}</>", value.to_string().unwrap_or(Self::CONV_ERROR.to_owned())))
				.map(|arg_v| Value::String(this.create_string(arg_v).unwrap()))
				.collect::<MultiValue>();
			luau_print.call::<()>(luau_multi_values)?;
			Ok(())
		})?)
	}

	fn global_version(&self, luau_globals: &Table) -> lResult<()> {
		let luau_info = luau_globals.get::<String>("_VERSION")?;
		luau_globals.raw_set("_VERSION", format!("{luau_info}, liblambdashell {}", Self::LIB_VERSION))
	}
}

pub struct LuauVm {
	vm: Luau,
	ps: Rc<RefCell<Ps>>
}
impl LuauVm {
	pub(crate) fn new(ps: Rc<RefCell<Ps>>) -> Self {
		Self { vm: Luau::new(), ps }
	}

	fn set_shell_globals(&self) -> lResult<()> {
		let luau_globals = self.vm.globals();
		self.global_shell(&luau_globals)?;
		self.global_terminal(&luau_globals)?;
		self.global_warn(&luau_globals)?;
		self.global_version(&luau_globals)?;
		luau_globals.raw_set("getfenv", mlua::Nil)?;
		luau_globals.raw_set("setfenv", mlua::Nil)?;
		self.vm.sandbox(true)?;
		Ok(())
	}

	pub fn exec(&self, source: String) {
		self.set_shell_globals().map_or_display_none(|()| self.vm.load(source).exec().map_or_luau_rt_err(Some));
	}
}