use mlua::{Result as lResult, Table};
use crossterm::style::Stylize;
use crate::vm::Vm;

macro_rules! foreground_styles_luau {
	($self:expr, $style_table:expr, $($color:ident)+) => {
		$(
			$style_table.set(stringify!($color).to_ascii_uppercase(), $self.0.create_function(|_, text: String| -> lResult<String> {
            	Ok(text.$color().to_string())
        	})?)?;
        )+
    };
}

macro_rules! background_styles_luau {
	($self:expr, $style_table:expr, $($color:ident)+) => {
		$(
			match stringify!($color).split_once("_") {
			    Some((_, color_name)) => $style_table.set(color_name.to_ascii_uppercase(), $self.0.create_function(|_, text: String| -> lResult<String> {
					Ok(text.$color().to_string())
        		})?)?,
			    None => panic!("Luau set error: {:?}. There was nothing to split from delimiter: \"_\"", stringify!($color)),
			}
        )+
    };
}

pub trait TerminalColors {
	fn var_terminal_colors_background(&self, style_table: &Table) -> lResult<()>;
	fn var_terminal_colors_foreground(&self, style_table: &Table) -> lResult<()>;
	fn var_terminal_text_styling(&self) -> lResult<Table>;
	fn var_terminal(&self) -> lResult<()>;
}
impl TerminalColors for Vm {
	fn var_terminal_colors_foreground(&self, style_table: &Table) -> lResult<()> {
		let foreground_table = self.0.create_table()?;
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
		style_table.set("FOREGROUND", foreground_table)?;
        Ok(())
	}

	fn var_terminal_colors_background(&self, style_table: &Table) -> lResult<()> {
		let background_table = self.0.create_table()?;
		background_styles_luau!(self, background_table,
			on_dark_grey   on_dark_red     on_dark_green on_dark_cyan
			on_dark_yellow on_dark_magenta on_dark_blue
			on_red   on_grey  on_black
		    on_green on_yellow
		    on_blue  on_magenta
		    on_cyan  on_white
		);
		style_table.set("BACKGROUND", background_table)?;
        Ok(())
	}

	fn var_terminal_text_styling(&self) -> lResult<Table> {
		let color_table = self.0.create_table()?;
		let style_table = self.0.create_table()?;
		self.var_terminal_colors_foreground(&style_table)?;
		self.var_terminal_colors_background(&style_table)?;
		color_table.set("STYLE", style_table)?;
		Ok(color_table)
	}

	fn var_terminal(&self) -> lResult<()> {
		let term_table = self.0.create_table()?;
		term_table.set("OUT", self.var_terminal_text_styling()?)?;
		self.0.globals().set("TERMINAL", &term_table)?;
		Ok(())
	}
}