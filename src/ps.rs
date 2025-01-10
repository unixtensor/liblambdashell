pub const DEFAULT_PS: &str = concat!("lambdashell-", env!("CARGO_PKG_VERSION"), " ");

#[derive(Debug)]
pub struct Ps(String);
impl Ps {
	pub const fn set(prompt: String) -> Self {
		Self(prompt)
	}
	//rustc: `std::string::String::as_str` is not yet stable as a const fn
	pub fn get(&self) -> &str {
		self.0.as_str()
	}
	pub fn modify(&mut self, prompt: String) {
		self.0 = prompt
	}
	pub fn display(&self) {
		print!("{}", self.0);
	}

	pub fn working_dir_name(&self) -> String {
		std::env::current_dir().map_or("?".to_owned(), |path| path.file_name().map_or("?".to_owned(), |name| {
			match name.to_os_string() == whoami::username_os() {
				true => "~".to_owned(),
				false => name.to_string_lossy().to_string(),
			}
		}))
	}
}