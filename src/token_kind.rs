use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
pub enum TokenKind {
    AmpersandAmpersand,
    Asterisk,
    At,
    Backtick,
    BangEquals,
    BarBar,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    ByteOrderMark,
    Colon,
    ColonEquals,
    Comma,
    Comment,
    Dedent,
    Dollar,
    Eof,
    Eol,
    Equals,
    EqualsEquals,
    EqualsTilde,
    Identifier,
    Indent,
    InterpolationEnd,
    InterpolationStart,
    ParenL,
    ParenR,
    Plus,
    QuestionMark,
    Slash,
    StringToken,
    Text,
    Unspecified,
    Whitespace,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match *self {
                AmpersandAmpersand => "'&&'",
                Asterisk => "'*'",
                At => "'@'",
                Backtick => "backtick",
                BangEquals => "'!='",
                BarBar => "'||'",
                BraceL => "'{'",
                BraceR => "'}'",
                BracketL => "'['",
                BracketR => "']'",
                ByteOrderMark => "byte order mark",
                Colon => "':'",
                ColonEquals => "':='",
                Comma => "','",
                Comment => "comment",
                Dedent => "dedent",
                Dollar => "'$'",
                Eof => "end of file",
                Eol => "end of line",
                Equals => "'='",
                EqualsEquals => "'=='",
                EqualsTilde => "'=~'",
                Identifier => "identifier",
                Indent => "indent",
                InterpolationEnd => "'}}'",
                InterpolationStart => "'{{'",
                ParenL => "'('",
                ParenR => "')'",
                Plus => "'+'",
                QuestionMark => "?",
                Slash => "'/'",
                StringToken => "string",
                Text => "command text",
                Unspecified => "unspecified",
                Whitespace => "whitespace",
            }
        )
    }
}
