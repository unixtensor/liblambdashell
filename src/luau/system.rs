use mlua::{Result as lResult, Table};
use whoami::fallible;
use crate::vm::LuauVm;

const DEFAULT_HOSTNAME: &str = "hostname";

pub trait PsPrompt {
	fn ps_prompt(&self) -> lResult<Table>;
}
impl PsPrompt for LuauVm {
	fn ps_prompt(&self) -> lResult<Table> {
		let prompt_table = self.0.create_table()?;
		let prompt_metatable = self.0.create_table()?;
		prompt_metatable.set("__index", self.0.create_function(|_, (lua_self, index): (String, String)| -> lResult<()> {
			println!("lua_self={} index={}", lua_self, index);
			Ok(())
		})?)?;
		prompt_metatable.set("__newindex", self.0.create_function(|_, _: String| -> lResult<String> {
			Ok("placeholder".to_owned())
		})?)?;
		prompt_table.set("__metatable", mlua::Nil)?;
		prompt_table.set_metatable(Some(prompt_metatable));
		Ok(prompt_table)
	}
}

pub trait System {
	fn sys_details(&self, shell: &Table) -> lResult<()>;
	fn shell_globals(&self, luau_globals: &Table) -> lResult<()>;
}
impl System for LuauVm {
	fn sys_details(&self, shell: &Table) -> lResult<()> {
		let system = self.0.create_table()?;
		system.set("DESKTOP_ENV", whoami::desktop_env().to_string())?;
		system.set("DEVICENAME", whoami::devicename().to_string())?;
		system.set("USERNAME", whoami::username().to_string())?;
		system.set("REALNAME", whoami::realname().to_string())?;
		system.set("PLATFORM", whoami::platform().to_string())?;
		system.set("DISTRO", whoami::distro().to_string())?;
		system.set("HOSTNAME", fallible::hostname().unwrap_or(DEFAULT_HOSTNAME.to_owned()))?;
		shell.set("SYSTEM", system)?;
		Ok(())
	}

	fn shell_globals(&self, luau_globals: &Table) -> lResult<()> {
		let shell = self.0.create_table()?;
		let ps_prompt = self.ps_prompt()?;
		self.sys_details(&shell)?;
		shell.set("PROMPT", ps_prompt)?;
		luau_globals.set("SHELL", shell)?;
		Ok(())
	}
}