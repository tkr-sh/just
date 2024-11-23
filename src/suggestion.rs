use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Suggestion<'src> {
  pub name: &'src str,
  pub target: Option<&'src str>,
}

impl<'src> Display for Suggestion<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Did you mean `{}`", self.name)?;
    if let Some(target) = self.target {
      write!(f, ", an alias for `{target}`")?;
    }
    write!(f, "?")
  }
}
