#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Delimiter {
    Brace,
    Bracket,
    Paren,
}

impl Delimiter {
    pub fn open(self) -> char {
        match self {
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Paren => '(',
        }
    }

    pub fn close(self) -> char {
        match self {
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Paren => ')',
        }
    }
}
