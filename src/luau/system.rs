use mlua::{Result as lResult, Table};
use whoami::fallible;
use crate::vm::Vm;

const DEFAULT_HOSTNAME: &str = "hostname";

pub trait System {
	fn details(&self, shell: &Table) -> lResult<()>;
	fn global_shell(&self) -> lResult<()>;
}
impl System for Vm {
	fn details(&self, shell: &Table) -> lResult<()> {
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

	fn global_shell(&self) -> lResult<()> {
		let shell = self.0.create_table()?;
		self.details(&shell)?;
		self.0.globals().set("SHELL", shell)?;
		Ok(())
	}
}