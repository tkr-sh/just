use super::*;

#[derive(Debug, Clone)]
pub struct Set<'src> {
  pub name: Name<'src>,
  pub value: Setting<'src>,
}

impl<'src> Keyed<'src> for Set<'src> {
  fn key(&self) -> &'src str {
    self.name.lexeme()
  }
}

impl<'src> Display for Set<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "set {} := {}", self.name, self.value)
  }
}
