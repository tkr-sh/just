use super::*;

/// A conditional expression operator.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ConditionalOperator {
    /// `==`
    Equality,
    /// `!=`
    Inequality,
    /// `=~`
    RegexMatch,
}

impl Display for ConditionalOperator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Equality => write!(f, "=="),
            Self::Inequality => write!(f, "!="),
            Self::RegexMatch => write!(f, "=~"),
        }
    }
}
