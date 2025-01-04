use const_format::formatcp;

pub const DEFAULT_PS: &str = formatcp!("lambdashell-{}", env!("CARGO_PKG_VERSION"));

struct Ps(String);
impl Ps {
	fn set(prompt: String) -> Self {
		Self(prompt)
	}

	fn working_dir_name(&self) -> String {
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

	fn display(&self) {
		print!("{}", self.0);
	}
}