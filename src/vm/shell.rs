use mlua::{Result as lResult, Table};
use whoami::fallible;

use crate::vm::LuauVm;

trait PsPrompt {
	fn ps_prompt(&self) -> lResult<Table>;
}
impl PsPrompt for LuauVm {
	fn ps_prompt(&self) -> lResult<Table> {
		let prompt_table = self.vm.create_table()?;
		let prompt_metatable = self.vm.create_table()?;
		let ps_owned = self.ps.to_owned();
		prompt_metatable.set("__index", self.vm.create_function(move |_, (s, s1): (Table, String)| -> lResult<String> {
			Ok(ps_owned.clone().get())
		})?)?;
		prompt_metatable.set("__newindex", self.vm.create_function(|_, _: String| -> lResult<String> {
			Ok("placeholder".to_owned())
		})?)?;
		// prompt_table.set("__metatable", mlua::Nil)?;
		prompt_table.set_metatable(Some(prompt_metatable));
		prompt_table.set_readonly(false);
		Ok(prompt_table)
	}
}

trait System {
	const DEFAULT_HOSTNAME: &str;
	fn sys_details(&self) -> lResult<Table>;
}
impl System for LuauVm {
	const DEFAULT_HOSTNAME: &str = "hostname";

	fn sys_details(&self) -> lResult<Table> {
		let system = self.vm.create_table()?;
		system.set("DESKTOP_ENV", whoami::desktop_env().to_string())?;
		system.set("DEVICENAME", whoami::devicename().to_string())?;
		system.set("USERNAME", whoami::username().to_string())?;
		system.set("REALNAME", whoami::realname().to_string())?;
		system.set("PLATFORM", whoami::platform().to_string())?;
		system.set("DISTRO", whoami::distro().to_string())?;
		system.set("HOSTNAME", fallible::hostname().unwrap_or(Self::DEFAULT_HOSTNAME.to_owned()))?;
		Ok(system)
	}
}

pub trait Shell {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()>;
}
impl Shell for LuauVm {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()> {
		let shell = self.vm.create_table()?;
		shell.set("SYSTEM", self.sys_details()?)?;
		shell.set("PROMPT", self.ps_prompt()?)?;
		luau_globals.set("SHELL", shell)?;
		Ok(())
	}
}