use super::*;

#[derive(PartialEq, Debug, Clone, Ord, Eq, PartialOrd)]
pub struct StringLiteral<'src> {
    pub cooked: String,
    pub expand: bool,
    pub kind: StringKind,
    pub raw: &'src str,
}

impl<'src> StringLiteral<'src> {
    pub fn from_raw(raw: &'src str) -> Self {
        Self {
            cooked: raw.into(),
            expand: false,
            kind: StringKind {
                delimiter: StringDelimiter::QuoteSingle,
                indented: false,
            },
            raw,
        }
    }
}

impl<'src> Display for StringLiteral<'src> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.expand {
            write!(f, "x")?;
        }

        write!(
            f,
            "{}{}{}",
            self.kind.delimiter(),
            self.raw,
            self.kind.delimiter()
        )
    }
}

impl<'src> Serialize for StringLiteral<'src> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.cooked)
    }
}
