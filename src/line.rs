use super::*;

/// A single line in a recipe body, consisting of any number of `Fragment`s.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Line<'src> {
    pub fragments: Vec<Fragment<'src>>,
    #[serde(skip)]
    pub number: usize,
}

impl<'src> Line<'src> {
    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    pub fn is_comment(&self) -> bool {
        matches!(
          self.fragments.first(),
          Some(Fragment::Text { token }) if token.lexeme().starts_with('#'),
        )
    }

    pub fn is_continuation(&self) -> bool {
        matches!(
          self.fragments.last(),
          Some(Fragment::Text { token }) if token.lexeme().ends_with('\\'),
        )
    }

    pub fn is_shebang(&self) -> bool {
        matches!(
          self.fragments.first(),
          Some(Fragment::Text { token }) if token.lexeme().starts_with("#!"),
        )
    }

    pub fn is_quiet(&self) -> bool {
        matches!(
          self.fragments.first(),
          Some(Fragment::Text { token })
            if token.lexeme().starts_with('@') || token.lexeme().starts_with("-@"),
        )
    }

    pub fn is_infallible(&self) -> bool {
        matches!(
          self.fragments.first(),
          Some(Fragment::Text { token })
            if token.lexeme().starts_with('-') || token.lexeme().starts_with("@-"),
        )
    }
}
