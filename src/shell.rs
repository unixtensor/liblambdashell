use crate::{ps, commands, rc};
use std::{fs, io::{self}};

pub struct Config {
    pub norc: bool
}

pub struct LambdaShell {
    terminating: bool,
    storage: Storage,
    config: Config,
}

struct Storage {
    pub command_exit_status: commands::ProcessExitStatus,
    pub ps1: String,
}

impl LambdaShell {
    pub fn create(config: Config) -> Self {
   		Self {
            storage: Storage {
                command_exit_status: None,
                ps1: ps::DEFAULT_PS.to_owned(),
            },
            terminating: false,
            config,
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_or_else(|read_error| println!("{read_error}"), |_size| {
	        let trimmed_input = input.trim();
	        match trimmed_input {
	            //special casey
	            "exit" => self.terminating = true,
	            _ => self.storage.command_exit_status = commands::Command::new(trimmed_input.to_owned()).exec()
	        };
        })
    }

    pub fn wait(&mut self) -> Result<(), io::Error> {
   		io::Write::flush(&mut io::stdout()).map_or_else(|flush_error| Err(flush_error), |()| {
	     	self.input();
			Ok(())
	    })
    }

    pub fn start(&mut self) {
	   	let rc_file = match self.config.norc {
	        true => rc::none(),
	        false => rc::config_file(),
	    };

     	ps::display(&self.storage.ps1);

        loop {
            match self.terminating {
                true => break,
                false => match self.wait() {
                    Ok(()) => ps::display(&self.storage.ps1),
                    Err(flush_error) => {
                        println!("{flush_error}");
                        break;
                    }
                },
            }
        }
    }
}
