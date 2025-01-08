use mlua::{Function, Result as lResult, Table};
use const_format::str_split;
use crossterm::style::Stylize;

use crate::vm::LuauVm;

macro_rules! foreground_styles_luau {
	($self:expr, $style_table:expr, $($color:ident)+) => {
		$(
			$style_table.set(stringify!($color).to_ascii_uppercase(), $self.vm.create_function(|_, text: String| -> lResult<String> {
            	Ok(text.$color().to_string())
        	})?)?;
        )+
    };
}
macro_rules! background_styles_luau {
	($self:expr, $style_table:expr, $($color:ident)+) => {
		$(
			$style_table.set(
				str_split!(stringify!($color), "_")[1..].join("_").to_ascii_uppercase(),
			$self.vm.create_function(|_, text: String| -> lResult<String> {
				Ok(text.$color().to_string())
        	})?)?;
        )+
    };
}

trait Colors {
	fn background(&self, term_out_table: &Table) -> lResult<()>;
	fn foreground(&self, term_out_table: &Table) -> lResult<()>;
}
impl Colors for LuauVm {
	fn background(&self, term_out_table: &Table) -> lResult<()> {
		let foreground_table = self.vm.create_table()?;
		foreground_styles_luau!(self, foreground_table,
			dark_grey   dark_red     dark_green dark_cyan
			dark_yellow dark_magenta dark_blue
			red  grey    black green yellow
		    blue magenta cyan  white
			underlined
			underline_dark_grey   underline_dark_red     underline_dark_green underline_dark_cyan
			underline_dark_yellow underline_dark_magenta underline_dark_blue  underline_red
			underline_grey        underline_black        underline_green      underline_yellow
			underline_blue        underline_magenta      underline_cyan       underline_white
			bold
		);
		term_out_table.set("FOREGROUND", foreground_table)
	}

	fn foreground(&self, term_out_table: &Table) -> lResult<()> {
		let background_table = self.vm.create_table()?;
		background_styles_luau!(self, background_table,
			on_dark_grey   on_dark_red     on_dark_green on_dark_cyan
			on_dark_yellow on_dark_magenta on_dark_blue
			on_red   on_grey  on_black
		    on_green on_yellow
		    on_blue  on_magenta
		    on_cyan  on_white
		);
		term_out_table.set("BACKGROUND", background_table)
	}
}

trait Write {
	fn write(&self) -> lResult<Function>;
	fn write_error(&self) -> lResult<Function>;
	fn write_error_ln(&self) -> lResult<Function>;
}
impl Write for LuauVm {
	fn write(&self) -> lResult<Function> {
		self.vm.create_function(|_, s: String| -> lResult<()> {
			print!("{s}");
			Ok(())
		})
	}

	fn write_error(&self) -> lResult<Function> {
		self.vm.create_function(|_, s: String| -> lResult<()> {
			eprint!("{s}");
			Ok(())
		})
	}

	fn write_error_ln(&self) -> lResult<Function> {
		self.vm.create_function(|_, s: String| -> lResult<()> {
			eprintln!("{s}");
			Ok(())
		})
	}
}

pub trait Terminal {
	fn out(&self) -> lResult<Table>;
	fn global_terminal(&self, luau_globals: &Table) -> lResult<()>;
}
impl Terminal for LuauVm {
	fn out(&self) -> lResult<Table> {
		let term_out_table = self.vm.create_table()?;
		self.background(&term_out_table)?;
		self.foreground(&term_out_table)?;
		Ok(term_out_table)
	}

	fn global_terminal(&self, luau_globals: &Table) -> lResult<()> {
		let term_table = self.vm.create_table()?;
		term_table.set("OUT", self.out()?)?;
		term_table.set("WRITE", self.write()?)?;
		term_table.set("WRITE_ERROR", self.write_error()?)?;
		term_table.set("WRITE_ERROR_LN", self.write_error_ln()?)?;
		luau_globals.set("TERMINAL", term_table)
	}
}