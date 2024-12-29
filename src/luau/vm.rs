use crate::VERSION;
use mlua::{
	Lua as Luau,
	Result as lResult,
	MultiValue,
};
use core::fmt;
use color_print::cprintln;

fn display_none<T, E>(err: E) -> Option<T>
where
	E: fmt::Display
{
	println!("{err}");
	None
}

fn luau_error<T>(err: mlua::Error) -> Option<T> {
	cprintln!("<bold>====</>\n<r><bold>[!]</> {err}</>\n<bold>====</>");
	None
}

fn luau_out(luau_args: MultiValue) -> String {
	let mut print: Vec<String> = Vec::new();
	luau_args.iter()
		.map(|arg| arg.to_string().unwrap_or("<SHELL CONVERSION ERROR>".to_owned()))
		.for_each(|arg| {
			if !print.is_empty() {
				print.push('\u{0009}'.to_string());
			};
			print.push(arg);
		}
	);
	print.concat()
}

trait Globals {
	fn print(&self) -> lResult<()>;
	fn printraw(&self) -> lResult<()>;
	fn version(&self) -> lResult<()>;
}
impl Globals for Vm {
	fn print(&self) -> lResult<()> {
		self.0.globals().set("print", self.0.create_function(|_this, args: MultiValue| -> lResult<()> {
			cprintln!("{}", luau_out(args));
			Ok(())
		})?)
	}
	fn printraw(&self) -> lResult<()> {
		self.0.globals().set("printraw", self.0.create_function(|_this, args: MultiValue| -> lResult<()> {
			println!("{}", luau_out(args));
			Ok(())
		})?)
	}
	fn version(&self) -> lResult<()> {
		let luau_info = self.0.globals().get::<String>("_VERSION")?;
		self.0.globals().set("_VERSION", format!("{}, liblambdashell {}", luau_info, VERSION))
	}
}

pub struct Vm(Luau);
impl Vm {
	pub fn new() -> Self {
		Self(Luau::new())
	}

	fn set_shell_globals(&self) -> mlua::Result<()> {
		self.print()?;
		self.printraw()?;
		self.version()?;
		self.0.globals().set("getfenv", mlua::Nil)?;
		self.0.globals().set("setfenv", mlua::Nil)?;
		self.0.sandbox(true)?;
		Ok(())
	}

	pub fn exec(&self, source: String) {
		self.set_shell_globals().map_or_else(|e| display_none(e), |()| {
			self.0.load(source).exec().map_or_else(|e| luau_error(e), |()| Some(()))
		});
	}
}