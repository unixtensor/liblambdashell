use mlua::{
	Lua as Luau,
	Result as lResult,
	MultiValue
};

fn new_instance() -> lResult<Luau> {
	let instance = Luau::new();
	instance.sandbox(true)?;
	instance.globals().set("getfenv", mlua::Nil)?;
	instance.globals().set("setfenv", mlua::Nil)?;
	Ok(instance)
}

fn out(args: MultiValue) -> String {
	let mut print: Vec<String> = Vec::new();
	let mut print_append = |v: String| {
		if !print.is_empty() {
			print.push(" ".to_owned());
		}
		print.push(v)
	};
	args.iter().for_each(|arg|
		arg.to_string().map_or(print_append("<SHELL CONVERSION ERROR>".to_owned()),
			|s_arg| print_append(s_arg)
		)
	);
	print.concat()
}

trait Globals {
	fn print(&self) -> lResult<()>;
}
impl Globals for Vm {
	fn print(&self) -> lResult<()> {
		self.0.globals().set("print", self.0.create_function(|_, args: MultiValue| -> lResult<()> {
			color_print::cprintln!("{}", out(args));
			Ok(())
		})?)?;
		self.0.globals().set("printraw", self.0.create_function(|_, args: MultiValue| -> lResult<()> {
			println!("{}", out(args));
			Ok(())
		})?)
	}
}

struct Vm(Luau);
impl Vm {
	pub fn new() -> Option<Self> {
		new_instance().map_or(None, |l| Some(Self(l)))
	}

	fn set_shell_globals(&self) -> mlua::Result<()> {
		todo!()
	}

	pub fn exec(&self, source: String) -> mlua::Result<()> {
		self.set_shell_globals().and(self.0.load(source).exec())
	}
}