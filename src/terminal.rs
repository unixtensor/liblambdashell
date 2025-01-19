use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal};
use std::io::{self, Write};
use thiserror::Error;

use crate::{commands::Command, history::History, session};

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
	fn key_enter(&mut self, history: &mut History) -> InputResult<()>;
	fn key_backspace(&mut self) -> InputResult<()>;
}
impl SpecificKeybinds for Processor {
	const TERM_ID_1: &str = "exit";

	fn key_ctrl(&mut self, input_key: KeyEvent, keycode: KeyCode) -> InputResult<()> {
		if input_key.modifiers.contains(KeyModifiers::CONTROL) {
			match keycode {
				KeyCode::Char('c') => Err(InputHandleError::Sigterm),
				_ => Ok(())
			}
		} else {
			self.render(Some(keycode.to_string()))
		}
	}

	fn key_enter(&mut self, history: &mut History) -> InputResult<()> {
		if self.0 == Self::TERM_ID_1 { return Err(InputHandleError::UserExit) };

		terminal::disable_raw_mode().map_err(InputHandleError::DisableRaw)?;
		Command::new(&self.0).exec(history);
		self.0.clear();
		Ok(())
	}

	fn key_backspace(&mut self) -> InputResult<()> {
		match self.0.pop() {
		    Some(_) => self.render(None),
		    None => {
				//the string is empty, do terminal beep
				Ok(())
			},
		}
	}
}

pub struct Processor(String);
impl Processor {
	pub const fn init() -> Self {
		Self(String::new())
	}

	pub fn render(&mut self, def: Option<String>) -> InputResult<()> {
		match def {
		    Some(def_string) => {
				self.0.push_str(&def_string);
				write!(io::stdout(), "{def_string}").map_err(InputHandleError::Write)?;
			},
		    None => {
				write!(io::stdout(), "{}", self.0).map_err(InputHandleError::Write)?
			}
		};
		io::stdout().flush().map_err(InputHandleError::Flush)
	}

	pub fn input_handler(&mut self, input_key: KeyEvent, history: &mut History) -> Option<()> {
		let input_handle = match input_key.code {
			KeyCode::Enter     => self.key_enter(history),
			KeyCode::Backspace => self.key_backspace(),
			KeyCode::Tab       => todo!(),
			KeyCode::Right     => todo!(),
			KeyCode::Left      => todo!(),
			KeyCode::Up        => todo!(),
			KeyCode::Down      => todo!(),
			keycode            => self.key_ctrl(input_key, keycode)
		};
		input_handle.map_or_else(|inp_err| match inp_err {
			InputHandleError::UserExit      => None,
		    InputHandleError::Sigterm       => self.render(Some("^C".to_owned())).ok(),
		    InputHandleError::Write(e)      => session::shell_error_none(e),
		    InputHandleError::Flush(e)      => session::shell_error_none(e),
		    InputHandleError::Key(e)        => session::shell_error_none(e),
			InputHandleError::DisableRaw(e) => session::shell_error_none(e),
			InputHandleError::EnableRaw(e)  => session::shell_error_none(e)
		}, Some)
	}

	fn input_mainthread(&mut self, history: &mut History) -> io::Result<()> {
		crossterm::execute!(io::stdout(), event::EnableBracketedPaste)?;
		loop {
			terminal::enable_raw_mode()?;
		    if let Event::Key(event) = event::read()? {
				if self.input_handler(event, history).is_none() { break Ok(()) }
			}
		}
	}

	pub fn input_processor(&mut self, history: &mut History) -> io::Result<()> {
		self.input_mainthread(history)?;
	    terminal::disable_raw_mode()?;
	    crossterm::execute!(io::stdout(), event::DisableBracketedPaste)
	}
}