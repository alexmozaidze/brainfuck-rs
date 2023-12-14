/// Tokens that could be encountered in a Brainfuck program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
	/// `+`
	Inc,
	/// `-`
	Dec,
	/// `>`
	Next,
	/// `<`
	Prev,
	/// `.`
	Print,
	/// `.`
	Read,
	/// `[`
	LoopStart,
	/// `]`
	LoopEnd,
}

impl Token {
	/// Tokenizes an input string, returning an iterator of tokens.
	pub fn tokenize(code: &str) -> impl Iterator<Item = Self> + '_ {
		code.chars().filter_map(|ch| match ch {
			'+' => Some(Self::Inc),
			'-' => Some(Self::Dec),
			'>' => Some(Self::Next),
			'<' => Some(Self::Prev),
			'.' => Some(Self::Print),
			',' => Some(Self::Read),
			'[' => Some(Self::LoopStart),
			']' => Some(Self::LoopEnd),
			_ => None,
		})
	}
}
