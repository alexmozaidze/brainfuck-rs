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
        code.bytes().filter_map(|ch| match ch {
            b'+' => Some(Self::Inc),
            b'-' => Some(Self::Dec),
            b'>' => Some(Self::Next),
            b'<' => Some(Self::Prev),
            b'.' => Some(Self::Print),
            b',' => Some(Self::Read),
            b'[' => Some(Self::LoopStart),
            b']' => Some(Self::LoopEnd),
            _ => None,
        })
    }
}
