use super::*;

/// A single function parameter
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Parameter<'src> {
    /// An optional default expression
    pub default: Option<Expression<'src>>,
    /// Export parameter as environment variable
    pub export: bool,
    /// The kind of parameter
    pub kind: ParameterKind,
    /// The parameter name
    pub name: Name<'src>,
}

impl<'src> ColorDisplay for Parameter<'src> {
    fn fmt(&self, f: &mut Formatter, color: Color) -> fmt::Result {
        if let Some(prefix) = self.kind.prefix() {
            write!(f, "{}", color.annotation().paint(prefix))?;
        }
        if self.export {
            write!(f, "$")?;
        }
        write!(f, "{}", color.parameter().paint(self.name.lexeme()))?;
        if let Some(ref default) = self.default {
            write!(f, "={}", color.string().paint(&default.to_string()))?;
        }
        Ok(())
    }
}
