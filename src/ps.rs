pub const DEFAULT_PS: &str = concat!("pse-", env!("CARGO_PKG_VERSION"), " ");

pub trait PsMut {
	fn get(&self) -> &str;
	fn modify(&mut self, prompt: String);
}
impl PsMut for Ps {
	#[inline]
	fn get(&self) -> &str {
		self.0.as_str()
	}
	#[inline]
	fn modify(&mut self, prompt: String) {
		self.0 = prompt
	}
}

#[derive(Debug)]
pub struct Ps(String);
impl Ps {
	pub const fn set(prompt: String) -> Self {
		Self(prompt)
	}
}