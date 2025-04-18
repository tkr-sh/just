use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
pub struct StringKind {
    pub delimiter: StringDelimiter,
    pub indented: bool,
}

impl StringKind {
    // Indented values must come before un-indented values, or else
    // `Self::from_token_start` will incorrectly return indented = false
    // for indented strings.
    const ALL: &'static [Self] = &[
        Self::new(StringDelimiter::Backtick, true),
        Self::new(StringDelimiter::Backtick, false),
        Self::new(StringDelimiter::QuoteDouble, true),
        Self::new(StringDelimiter::QuoteDouble, false),
        Self::new(StringDelimiter::QuoteSingle, true),
        Self::new(StringDelimiter::QuoteSingle, false),
    ];

    const fn new(delimiter: StringDelimiter, indented: bool) -> Self {
        Self {
            delimiter,
            indented,
        }
    }

    pub fn delimiter(self) -> &'static str {
        match (self.delimiter, self.indented) {
            (StringDelimiter::Backtick, false) => "`",
            (StringDelimiter::Backtick, true) => "```",
            (StringDelimiter::QuoteDouble, false) => "\"",
            (StringDelimiter::QuoteDouble, true) => "\"\"\"",
            (StringDelimiter::QuoteSingle, false) => "'",
            (StringDelimiter::QuoteSingle, true) => "'''",
        }
    }

    pub fn delimiter_len(self) -> usize {
        self.delimiter().len()
    }

    pub fn token_kind(self) -> TokenKind {
        match self.delimiter {
            StringDelimiter::QuoteDouble | StringDelimiter::QuoteSingle => TokenKind::StringToken,
            StringDelimiter::Backtick => TokenKind::Backtick,
        }
    }

    pub fn unterminated_error_kind(self) -> CompileErrorKind<'static> {
        match self.delimiter {
            StringDelimiter::QuoteDouble | StringDelimiter::QuoteSingle => {
                CompileErrorKind::UnterminatedString
            },
            StringDelimiter::Backtick => CompileErrorKind::UnterminatedBacktick,
        }
    }

    pub fn processes_escape_sequences(self) -> bool {
        match self.delimiter {
            StringDelimiter::QuoteDouble => true,
            StringDelimiter::Backtick | StringDelimiter::QuoteSingle => false,
        }
    }

    pub fn indented(self) -> bool {
        self.indented
    }

    pub fn from_string_or_backtick(token: Token) -> CompileResult<Self> {
        Self::from_token_start(token.lexeme()).ok_or_else(|| {
            token.error(CompileErrorKind::Internal {
                message: "StringKind::from_token: Expected String or Backtick".to_owned(),
            })
        })
    }

    pub fn from_token_start(token_start: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .find(|&&kind| token_start.starts_with(kind.delimiter()))
            .copied()
    }
}
