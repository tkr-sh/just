use super::*;

#[derive(Debug)]
pub struct Compilation<'src> {
  pub asts: HashMap<PathBuf, Ast<'src>>,
  pub justfile: Justfile<'src>,
  pub root: PathBuf,
  pub srcs: HashMap<PathBuf, &'src str>,
}

impl<'src> Compilation<'src> {
  pub fn root_ast(&self) -> &Ast<'src> {
    self.asts.get(&self.root).unwrap()
  }

  pub fn root_src(&self) -> &'src str {
    self.srcs.get(&self.root).unwrap()
  }
}
