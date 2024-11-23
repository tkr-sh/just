use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct UnresolvedDependency<'src> {
  pub recipe: Name<'src>,
  pub arguments: Vec<Expression<'src>>,
}

impl<'src> Display for UnresolvedDependency<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    if self.arguments.is_empty() {
      write!(f, "{}", self.recipe)
    } else {
      write!(f, "({}", self.recipe)?;

      for argument in &self.arguments {
        write!(f, " {argument}")?;
      }

      write!(f, ")")
    }
  }
}
