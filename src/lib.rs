pub mod session;
pub mod commands;
pub mod ps;
pub mod rc;
pub mod vm;

pub trait MapDisplay<T, E: std::fmt::Display> {
	fn map_or_display<F: FnOnce(T)>(self, f: F);
	fn map_or_display_none<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R>;
}
impl<T, E: std::fmt::Display> MapDisplay<T, E> for Result<T, E> {
	///Map and display an error
	#[inline]
	fn map_or_display<F: FnOnce(T)>(self, f: F) {
		self.map_or_else(|e| color_print::ceprintln!("<bold,r>[!]:</> {e}"), f)
	}
	///Map and display an error but return `None`
	#[inline]
	fn map_or_display_none<R, F: FnOnce(T) -> Option<R>>(self, f: F) -> Option<R> {
		self.map_or_else(|e| { color_print::ceprintln!("<bold,r>[!]:</> {e}"); None }, f)
	}
}