use mlua::{Result as lResult, Table};
use crossterm::style::Stylize;
use crate::vm::Vm;

macro_rules! term_colors_luau {
    ($self:expr, $colors_table:expr, $($color:ident)+) => {
        $(
	    	let $color = $self.0.create_function(|_this, text: String| -> lResult<String> {
	            Ok(text.$color().to_string())
	        })?;
			$colors_table.set(stringify!($color).to_ascii_uppercase(), $color)?;
        )+
    };
}

pub trait TerminalColors {
	fn var_terminal_colors(&self, colors_table: &Table) -> lResult<()>;
	fn var_terminal_style(&self) -> lResult<Table>;
	fn var_terminal(&self) -> lResult<()>;
}
impl TerminalColors for Vm {
	fn var_terminal_colors(&self, colors_table: &Table) -> lResult<()> {
		term_colors_luau!(self, colors_table,
			dark_grey   dark_red     dark_green dark_cyan
			dark_yellow dark_magenta dark_blue
			red   grey  black
		    green yellow
		    blue  magenta
		    cyan  white
		);
		Ok(())
	}

	fn var_terminal_style(&self) -> lResult<Table> {
		let color_table = self.0.create_table()?;
		let style = self.0.create_table()?;
		self.var_terminal_colors(&style)?;
		color_table.set("STYLE", &style)?;
		Ok(color_table)
	}

	fn var_terminal(&self) -> lResult<()> {
		let term_table = self.0.create_table()?;
		let style_table = self.var_terminal_style()?;
		term_table.set("OUT", style_table)?;
		self.0.globals().set("TERMINAL", &term_table)?;
		Ok(())
	}
}