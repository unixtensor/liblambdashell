use mlua::{
	Function, Lua as Luau, MultiValue, Result as lResult, Table, Value
};
use crate::vm::{shell::System, terminal::Terminal};
use crate::VERSION;
use color_print::{cformat, cprintln};
use core::fmt;

mod shell;
mod terminal;
mod alias;

trait Helpers {
	fn option_display_none<T, E: fmt::Display>(&self, err: E) -> Option<T>;
	fn luau_error<T>(&self, err: mlua::Error) -> Option<T>;
}
impl Helpers for LuauVm {
	fn option_display_none<T, E: fmt::Display>(&self, err: E) -> Option<T> {
		println!("{err}");
		None
	}

	fn luau_error<T>(&self, err: mlua::Error) -> Option<T> {
		cprintln!("<bold>====</>\n<r><bold>[!]:</> {err}</>\n<bold>====</>");
		None
	}
}

trait Globals {
	fn global_warn(&self, luau_globals: &Table) -> lResult<()>;
	fn global_version(&self, luau_globals: &Table) -> lResult<()>;
}
impl Globals for LuauVm {
	fn global_warn(&self, luau_globals: &Table) -> lResult<()> {
		let luau_print = luau_globals.get::<Function>("print")?;
		luau_globals.set("warn", self.0.create_function(move |this, args: MultiValue| -> lResult<()> {
			let luau_multi_values = args.into_iter()
				.map(|value| cformat!("<y>{}</>", value.to_string().unwrap_or("<SHELL CONVERSION ERROR>".to_owned())))
				.map(|arg_v| Value::String(this.create_string(arg_v).unwrap()))
				.collect::<MultiValue>();
			luau_print.call::<()>(luau_multi_values).unwrap();
			Ok(())
		})?)
	}

	fn global_version(&self, luau_globals: &Table) -> lResult<()> {
		let luau_info = luau_globals.get::<String>("_VERSION")?;
		luau_globals.set("_VERSION", format!("{}, liblambdashell {}", luau_info, VERSION))
	}
}

pub struct LuauVm(pub Luau);
impl LuauVm {
	pub(crate) fn new() -> Self {
		Self(Luau::new())
	}

	fn set_shell_globals(&self) -> lResult<()> {
		let luau_globals = self.0.globals();
		self.global_warn(&luau_globals)?;
		self.global_version(&luau_globals)?;
		self.global_terminal(&luau_globals)?;
		self.shell_globals(&luau_globals)?;
		luau_globals.set("getfenv", mlua::Nil)?;
		luau_globals.set("setfenv", mlua::Nil)?;
		self.0.sandbox(true)?;
		Ok(())
	}

	pub fn exec(&self, source: String) {
		match self.set_shell_globals() {
		    Ok(()) => self.0.load(source).exec().map_or_else(|exec_err| self.luau_error(exec_err), Some),
		    Err(globals_err) => self.option_display_none(globals_err),
		};
	}
}