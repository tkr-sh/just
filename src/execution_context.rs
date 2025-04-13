use super::*;

#[derive(Copy, Clone)]
pub struct ExecutionContext<'src: 'run, 'run> {
    pub config: &'run Config,
    pub dotenv: &'run BTreeMap<String, String>,
    pub module: &'run Justfile<'src>,
    pub scope: &'run Scope<'src, 'run>,
    pub search: &'run Search,
}

impl<'src: 'run, 'run> ExecutionContext<'src, 'run> {
    pub fn working_directory(&self) -> PathBuf {
        let base = if self.module.is_submodule() {
            &self.module.working_directory
        } else {
            &self.search.working_directory
        };

        if let Some(setting) = &self.module.settings.working_directory {
            base.join(setting)
        } else {
            base.into()
        }
    }
}
