use super::*;

#[derive(PartialEq, Debug, Serialize)]
pub struct Dependency<'src> {
  pub arguments: Vec<Expression<'src>>,
  #[serde(serialize_with = "keyed::serialize")]
  pub recipe: Rc<Recipe<'src>>,
}

impl<'src> Display for Dependency<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    if self.arguments.is_empty() {
      write!(f, "{}", self.recipe.name())
    } else {
      write!(f, "({}", self.recipe.name())?;

      for argument in &self.arguments {
        write!(f, " {argument}")?;
      }

      write!(f, ")")
    }
  }
}
