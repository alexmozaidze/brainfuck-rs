/// A trait made for strings to strip shebang out.
///
/// # Usage
///
/// ```
/// // Don't forget to import the trait before using its methods
/// use brainfuck_rs::utils::StripShebang;
///
/// let code = "#!/usr/bin/env brainfuck-rs
/// [Some Brainfuck code...]
/// ";
///
/// let code_but_without_shebang = code.strip_shebang();
/// ```
pub trait StripShebang: AsRef<str> {
	/// Strips shebang from a string, in case it exists.
	fn strip_shebang(&self) -> &str {
		let input = self.as_ref();

		if !input.starts_with("#!") {
			return input;
		}

		let index = input.find('\n').unwrap_or(input.len());

		&input[index..]
	}
}

impl StripShebang for String {}
impl StripShebang for &str {}
