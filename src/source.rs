use super::*;

#[derive(Debug)]
pub struct Source<'src> {
  pub file_depth: u32,
  pub file_path: Vec<PathBuf>,
  pub import_offsets: Vec<usize>,
  pub namepath: Namepath<'src>,
  pub path: PathBuf,
  pub working_directory: PathBuf,
}

impl<'src> Source<'src> {
  pub fn root(path: &Path) -> Self {
    Self {
      file_depth: 0,
      file_path: vec![path.into()],
      import_offsets: Vec::new(),
      namepath: Namepath::default(),
      path: path.into(),
      working_directory: path.parent().unwrap().into(),
    }
  }

  pub fn import(&self, path: PathBuf, import_offset: usize) -> Self {
    Self {
      file_depth: self.file_depth + 1,
      file_path: self
        .file_path
        .clone()
        .into_iter()
        .chain(iter::once(path.clone()))
        .collect(),
      import_offsets: self
        .import_offsets
        .iter()
        .copied()
        .chain(iter::once(import_offset))
        .collect(),
      namepath: self.namepath.clone(),
      path,
      working_directory: self.working_directory.clone(),
    }
  }

  pub fn module(&self, name: Name<'src>, path: PathBuf) -> Self {
    Self {
      file_depth: self.file_depth + 1,
      file_path: self
        .file_path
        .clone()
        .into_iter()
        .chain(iter::once(path.clone()))
        .collect(),
      import_offsets: Vec::new(),
      namepath: self.namepath.join(name),
      path: path.clone(),
      working_directory: path.parent().unwrap().into(),
    }
  }
}
