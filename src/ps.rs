use const_format::formatcp;
use color_print::{cformat, cprint};

pub const DEFAULT_PS: &str = formatcp!("lambdashell-{}", env!("CARGO_PKG_VERSION"));

pub fn working_dir_name() -> String {
	std::env::current_dir().map_or("?".to_owned(), |path| {
		path.file_name().map_or("?".to_owned(), |name| {
			let name_os_string = name.to_os_string();
			match name_os_string == whoami::username_os() && name_os_string != "root" {
				true => "~".to_owned(),
				false => name.to_string_lossy().to_string(),
			}
		})
	})
}

pub fn display(ps1: &String) {
	let working_dir_name = cformat!(" <bold>{}</> ", working_dir_name());
	cprint!("{}{}λ ", ps1, working_dir_name);
}
