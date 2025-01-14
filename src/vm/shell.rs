use mlua::{Lua as Luau, MetaMethod, Result as lResult, Table, UserData, UserDataFields, UserDataMethods};
use std::{cell::RefCell, rc::Rc};
use whoami::fallible;

use crate::{ps::Ps, vm::LuauVm};

const DEFAULT_HOSTNAME: &str = "hostname";

fn luau_sys_details(luau: &Luau) -> lResult<Table> {
	let system = luau.create_table()?;
	system.raw_set("DESKTOP_ENV", whoami::desktop_env().to_string())?;
	system.raw_set("DEVICENAME", whoami::devicename().to_string())?;
	system.raw_set("USERNAME", whoami::username().to_string())?;
	system.raw_set("REALNAME", whoami::realname().to_string())?;
	system.raw_set("PLATFORM", whoami::platform().to_string())?;
	system.raw_set("DISTRO", whoami::distro().to_string())?;
	system.raw_set("ARCH", whoami::arch().to_string())?;
	system.raw_set("HOSTNAME", fallible::hostname().unwrap_or(DEFAULT_HOSTNAME.to_owned()))?;
	Ok(system)
}

struct Shell(Rc<RefCell<Ps>>);
impl UserData for Shell {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("PROMPT", |_, this| Ok(this.0.borrow().get().to_owned()));
		fields.add_field_method_get("SYSTEM", |luau, _| luau_sys_details(luau));
	}

	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_meta_method_mut(MetaMethod::NewIndex, |_, this, (tindex, tvalue): (String, String)| -> lResult<()> {
			if tindex == "PROMPT" {
				this.0.borrow_mut().modify(tvalue);
			}
			Ok(())
		});
	}
}

pub trait ShellGlobal {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()>;
}
impl ShellGlobal for LuauVm {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()> {
		luau_globals.raw_set("SHELL", Shell(Rc::clone(&self.ps)))?;
		Ok(())
	}
}