use mlua::{
	Lua as Luau,
	Result as lResult
};
use crate::VERSION;
use crate::terminal::TerminalColors;
use crate::sytem::System;
use core::fmt;
use color_print::cprintln;

fn display_none<T, E: fmt::Display>(err: E) -> Option<T> {
	println!("{err}");
	None
}

fn luau_error<T>(err: mlua::Error) -> Option<T> {
	cprintln!("<bold>====</>\n<r><bold>[!]</> {err}</>\n<bold>====</>");
	None
}

trait Globals {
	fn version(&self) -> lResult<()>;
}
impl Globals for Vm {
	fn version(&self) -> lResult<()> {
		let luau_info = self.0.globals().get::<String>("_VERSION")?;
		self.0.globals().set("_VERSION", format!("{}, liblambdashell {}", luau_info, VERSION))
	}
}

pub struct Vm(pub Luau);
impl Vm {
	pub fn new() -> Self {
		Self(Luau::new())
	}

	fn set_shell_globals(&self) -> mlua::Result<()> {
		self.version()?;
		self.terminal()?;
		self.global_shell()?;
		self.0.globals().set("getfenv", mlua::Nil)?;
		self.0.globals().set("setfenv", mlua::Nil)?;
		self.0.sandbox(true)?;
		Ok(())
	}

	pub fn exec(&self, source: String) {
		self.set_shell_globals().map_or_else(display_none, |()| {
			self.0.load(source).exec().map_or_else(luau_error, Some)
		});
	}
}