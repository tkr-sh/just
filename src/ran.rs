use super::*;

#[derive(Default)]
pub struct Ran<'src>(BTreeMap<Namepath<'src>, BTreeSet<Vec<String>>>);

impl<'src> Ran<'src> {
  pub fn has_run(&self, recipe: &Namepath<'src>, arguments: &[String]) -> bool {
    self
      .0
      .get(recipe)
      .is_some_and(|ran| ran.contains(arguments))
  }

  pub fn ran(&mut self, recipe: &Namepath<'src>, arguments: Vec<String>) {
    self.0.entry(recipe.clone()).or_default().insert(arguments);
  }
}
