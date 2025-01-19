use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal};
use std::io::{self, Write};
use thiserror::Error;

use crate::{commands::Command, session::{self, Pse}};

#[derive(Debug, Error)]
pub enum InputHandleError {
	#[error("UserExit")]
	UserExit,
	#[error("Sigterm")]
	Sigterm,
	#[error("Render failure: {0}")]
	Write(io::Error),
	#[error("Flush failure: {0}")]
	Flush(io::Error),
	#[error("Disabling the terminal's raw mode failed: {0}")]
	EnableRaw(io::Error),
	#[error("Enabling the terminal's raw mode failed: {0}")]
	DisableRaw(io::Error),
	#[error("key input failure: {0}")]
	Key(KeyCode),
}
type InputResult<T> = Result<T, InputHandleError>;

trait SpecificKeybinds {
	const TERM_ID_1: &str;
	fn key_ctrl(&mut self, input_key: KeyEvent, keycode: KeyCode) -> InputResult<()>;
	fn key_enter(&mut self) -> InputResult<()>;
	fn key_backspace(&mut self) -> InputResult<()>;
}
impl SpecificKeybinds for Pse {
	const TERM_ID_1: &str = "exit";

	fn key_ctrl(&mut self, input_key: KeyEvent, keycode: KeyCode) -> InputResult<()> {
		if input_key.modifiers.contains(KeyModifiers::CONTROL) {
			match keycode {
				KeyCode::Char('c') => Err(InputHandleError::Sigterm),
				_ => Ok(())
			}
		} else {
			self.term_render(Some(keycode.to_string()))
		}
	}

	fn key_enter(&mut self) -> InputResult<()> {
		if self.rt.input == Self::TERM_ID_1 { return Err(InputHandleError::UserExit) };

		terminal::disable_raw_mode().map_err(InputHandleError::DisableRaw)?;
		Command::new(&self.rt.input).exec(&mut self.history);
		self.rt.input.clear();
		Ok(())
	}

	fn key_backspace(&mut self) -> InputResult<()> {
		match self.rt.input.pop() {
		    Some(_) => self.term_render(None),
		    None => {
				//the string is empty, do terminal beep
				Ok(())
			},
		}
	}
}

pub trait TermProcessor {
	fn term_render(&mut self, def: Option<String>) -> InputResult<()>;
	fn term_input_handler(&mut self, input_key: KeyEvent) -> Option<()>;
	fn term_input_mainthread(&mut self) -> io::Result<()>;
	fn term_input_processor(&mut self) -> io::Result<()>;
}
impl TermProcessor for Pse {
	fn term_render(&mut self, def: Option<String>) -> InputResult<()> {
		match def {
		    Some(def_string) => {
				self.rt.input.push_str(&def_string);
				write!(io::stdout(), "{def_string}").map_err(InputHandleError::Write)?;
			},
		    None => {
				write!(io::stdout(), "{}", self.rt.input).map_err(InputHandleError::Write)?
			}
		};
		io::stdout().flush().map_err(InputHandleError::Flush)
	}

	fn term_input_handler(&mut self, input_key: KeyEvent) -> Option<()> {
		let input_handle = match input_key.code {
			KeyCode::Enter     => self.key_enter(),
			KeyCode::Backspace => self.key_backspace(),
			KeyCode::Tab       => todo!(),
			KeyCode::Right     => todo!(),
			KeyCode::Left      => todo!(),
			KeyCode::Up        => todo!(),
			KeyCode::Down      => todo!(),
			keycode            => self.key_ctrl(input_key, keycode)
		};
		input_handle.map_or_else(|inp_err| match inp_err {
			InputHandleError::UserExit => None,
		    InputHandleError::Sigterm => self.term_render(Some("^C".to_owned())).ok(),
			input_err => session::shell_error_none(input_err)
		}, Some)
	}

	fn term_input_mainthread(&mut self) -> io::Result<()> {
		crossterm::execute!(io::stdout(), event::EnableBracketedPaste)?;
		loop {
			terminal::enable_raw_mode()?;
		    if let Event::Key(event) = event::read()? {
				if self.term_input_handler(event).is_none() { break Ok(()) }
			}
		}
	}

	fn term_input_processor(&mut self) -> io::Result<()> {
		self.term_input_mainthread()?;
	    terminal::disable_raw_mode()?;
	    crossterm::execute!(io::stdout(), event::DisableBracketedPaste)
	}
}