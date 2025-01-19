use mlua::{Function, MultiValue, Result as lResult, Table, Value};
use color_print::cformat;
use shell::ShellGlobal;
use terminal::TerminalGlobal;
use core::fmt;

use crate::session::{Pse, MapDisplay};

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
			color_print::ceprintln!("<bold>====</>\n<r><bold>[!]:</> {luau_rt_err}</>\n<bold>====</>");
			None
		}, f)
	}
}

trait VmGlobals {
	const LIB_VERSION: &str;
	const CONV_ERROR: &str;
	const LIB_NAME: &str;
	fn vm_glob_warn(&self, luau_globals: &Table) -> lResult<()>;
	fn vm_glob_version(&self, luau_globals: &Table) -> lResult<()>;
}
impl VmGlobals for Pse {
	const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
	const LIB_NAME: &str = env!("CARGO_PKG_NAME");
	const CONV_ERROR: &str = "<SHELL CONVERSION ERROR>";

	fn vm_glob_warn(&self, luau_globals: &Table) -> lResult<()> {
		let luau_print = luau_globals.get::<Function>("print")?;
		luau_globals.raw_set("warn", self.rt.vm.create_function(move |this, args: MultiValue| -> lResult<()> {
			let luau_multi_values = args.into_iter()
				.map(|value| cformat!("<bold,y>{}</>", value.to_string().unwrap_or(Self::CONV_ERROR.to_owned())))
				.map(|arg_v| Value::String(this.create_string(arg_v).unwrap()))
				.collect::<MultiValue>();
			luau_print.call::<()>(luau_multi_values)?;
			Ok(())
		})?)
	}

	fn vm_glob_version(&self, luau_globals: &Table) -> lResult<()> {
		let luau_info = luau_globals.get::<String>("_VERSION")?;
		luau_globals.raw_set("_VERSION", format!("{luau_info}, {} {}", Self::LIB_NAME, Self::LIB_VERSION))
	}
}

pub trait LuauVm {
	fn vm_setglobs(&self) -> lResult<()>;
	fn vm_exec(&self, source: String);
}
impl LuauVm for Pse {
	fn vm_setglobs(&self) -> lResult<()> {
		let luau_globals = self.rt.vm.globals();
		self.vm_glob_shell(&luau_globals)?;
		self.vm_glob_terminal(&luau_globals)?;
		self.vm_glob_warn(&luau_globals)?;
		self.vm_glob_version(&luau_globals)?;

		luau_globals.raw_set("getfenv", mlua::Nil)?;
		luau_globals.raw_set("setfenv", mlua::Nil)?;
		self.rt.vm.enable_jit(self.config.vm.jit);
		self.rt.vm.sandbox(true)
	}

	fn vm_exec(&self, source: String) {
		self.vm_setglobs().map_or_display_none(|()| self.rt.vm.load(source).exec().map_or_luau_rt_err(Some));
	}
}