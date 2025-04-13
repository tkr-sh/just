use super::*;

/// A binding of `name` to `value`
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Binding<'src, V = String> {
    #[serde(skip)]
    pub constant: bool,
    pub export: bool,
    #[serde(skip)]
    pub file_depth: u32,
    pub name: Name<'src>,
    pub private: bool,
    pub value: V,
}

impl<'src, V> Keyed<'src> for Binding<'src, V> {
    fn key(&self) -> &'src str {
        self.name.lexeme()
    }
}
