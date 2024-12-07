use const_format::formatcp;
use color_print::{cformat, cprint};

pub const DEFAULT_PS: &str = formatcp!("lambdashell-{}", env!("CARGO_PKG_VERSION"));

pub fn working_dir_name() -> String {
    match std::env::current_dir() {
        Ok(pathbuf) => match pathbuf.file_name() {
            Some(name) => {
                let name_os_string = name.to_os_string();
                match name_os_string == whoami::username_os() && name_os_string != "root" {
                    true => "~".to_string(),
                    false => name.to_string_lossy().to_string(),
                }
            }
            None => "?".to_string(),
        },
        Err(_) => "?".to_string(),
    }
}

pub fn display(ps1: &String) {
    // let exit_status = shell_storage.command_exit_status.map(|s| format!(" [{s}] ")).unwrap_or(" ".to_string());
    let working_dir_name = cformat!(" <bold>{}</> ", working_dir_name());
    cprint!("{}{}λ ", ps1, working_dir_name);
}
