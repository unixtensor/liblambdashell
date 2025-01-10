use mlua::{MetaMethod, Result as lResult, Table, UserData, UserDataFields, UserDataMethods};
use std::{cell::RefCell, rc::Rc};
use whoami::fallible;

use crate::{ps::Ps, vm::LuauVm};

trait System {
	const DEFAULT_HOSTNAME: &str;
	fn sys_details(&self) -> lResult<Table>;
}
impl System for LuauVm {
	const DEFAULT_HOSTNAME: &str = "hostname";

	fn sys_details(&self) -> lResult<Table> {
		let system = self.vm.create_table()?;
		system.raw_set("DESKTOP_ENV", whoami::desktop_env().to_string())?;
		system.raw_set("DEVICENAME", whoami::devicename().to_string())?;
		system.raw_set("USERNAME", whoami::username().to_string())?;
		system.raw_set("REALNAME", whoami::realname().to_string())?;
		system.raw_set("PLATFORM", whoami::platform().to_string())?;
		system.raw_set("DISTRO", whoami::distro().to_string())?;
		system.raw_set("ARCH", whoami::arch().to_string())?;
		system.raw_set("HOSTNAME", fallible::hostname().unwrap_or(Self::DEFAULT_HOSTNAME.to_owned()))?;
		Ok(system)
	}
}

struct ShellUserdata(Rc<RefCell<Ps>>);
impl UserData for ShellUserdata {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("PROMPT", |_, this| Ok(this.0.borrow().get().to_owned()));
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

pub trait Shell {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()>;
}
impl Shell for LuauVm {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()> {
		// let shell = self.vm.create_table()?;
		// shell.set("SYSTEM", self.sys_details()?)?;
		// shell.set("PROMPT", self.ps_prompt()?)?;
		luau_globals.set("SHELL", ShellUserdata(Rc::clone(&self.ps)))?;
		Ok(())
	}
}