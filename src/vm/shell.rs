use mlua::{Result as lResult, Table, Value};
use whoami::fallible;

use crate::vm::LuauVm;

const DEFAULT_HOSTNAME: &str = "hostname";

trait PsPrompt {
	fn ps_prompt(&self) -> lResult<Table>;
}
impl PsPrompt for LuauVm {
	fn ps_prompt(&self) -> lResult<Table> {
		let prompt_table = self.0.create_table()?;
		let prompt_metatable = self.0.create_table()?;
		prompt_metatable.set("__index", self.0.create_function(|_, (table, index): (Table, Value)| -> lResult<String> {
			table.raw_get::<String>(index)
		})?)?;
		prompt_metatable.set("__newindex", self.0.create_function(|_, _: String| -> lResult<String> {
			Ok("placeholder".to_owned())
		})?)?;
		prompt_table.set("__metatable", mlua::Nil)?;
		prompt_table.set_metatable(Some(prompt_metatable));
		prompt_table.set_readonly(false);
		Ok(prompt_table)
	}
}

trait System {
	fn sys_details(&self) -> lResult<Table>;
}
impl System for LuauVm {
	fn sys_details(&self) -> lResult<Table> {
		let system = self.0.create_table()?;
		system.set("DESKTOP_ENV", whoami::desktop_env().to_string())?;
		system.set("DEVICENAME", whoami::devicename().to_string())?;
		system.set("USERNAME", whoami::username().to_string())?;
		system.set("REALNAME", whoami::realname().to_string())?;
		system.set("PLATFORM", whoami::platform().to_string())?;
		system.set("DISTRO", whoami::distro().to_string())?;
		system.set("HOSTNAME", fallible::hostname().unwrap_or(DEFAULT_HOSTNAME.to_owned()))?;
		Ok(system)
	}
}

pub trait Shell {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()>;
}
impl Shell for LuauVm {
	fn global_shell(&self, luau_globals: &Table) -> lResult<()> {
		let shell = self.0.create_table()?;
		let ps_prompt = self.ps_prompt()?;
		shell.set("SYSTEM", self.sys_details()?)?;
		shell.set("PROMPT", ps_prompt)?;
		luau_globals.set("SHELL", shell)?;
		Ok(())
	}
}