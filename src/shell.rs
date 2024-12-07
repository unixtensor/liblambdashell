use crate::{ps, commands};
use std::io;

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
                ps1: ps::DEFAULT_PS.to_string(),
            },
            terminating: false,
            config,
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_size) => {
                let trimmed_input = input.trim();
                match trimmed_input {
                    //special casey
                    "exit" => self.terminating = true,
                    _ => self.storage.command_exit_status = commands::Command::new(trimmed_input.to_string()).exec()
                };
            }
            Err(read_error) => println!("{read_error}"),
        };
    }

    pub fn wait(&mut self) -> Result<(), io::Error> {
        match io::Write::flush(&mut io::stdout()) {
            Ok(()) => {
                self.input();
                Ok(())
            }
            Err(flush_error) => {
                println!("{flush_error}");
                Err(flush_error)
            }
        }
    }

    pub fn start(&mut self) {
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
