use super::*;

#[derive(Debug)]
pub struct Scope<'src: 'run, 'run> {
    parent: Option<&'run Self>,
    bindings: Table<'src, Binding<'src, String>>,
}

impl<'src, 'run> Scope<'src, 'run> {
    pub fn child(&'run self) -> Self {
        Self {
            parent: Some(self),
            bindings: Table::new(),
        }
    }

    pub fn root() -> Self {
        let mut root = Self {
            parent: None,
            bindings: Table::new(),
        };

        for (key, value) in constants() {
            root.bind(Binding {
                constant: true,
                export: false,
                file_depth: 0,
                name: Name {
                    token: Token {
                        column: 0,
                        kind: TokenKind::Identifier,
                        length: key.len(),
                        line: 0,
                        offset: 0,
                        path: Path::new("PRELUDE"),
                        src: key,
                    },
                },
                private: false,
                value: (*value).into(),
            });
        }

        root
    }

    pub fn bind(&mut self, binding: Binding<'src>) {
        self.bindings.insert(binding);
    }

    pub fn bound(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn value(&self, name: &str) -> Option<&str> {
        if let Some(binding) = self.bindings.get(name) {
            Some(binding.value.as_ref())
        } else {
            self.parent?.value(name)
        }
    }

    pub fn bindings(&self) -> impl Iterator<Item = &Binding<String>> {
        self.bindings.values()
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.bindings.keys().copied()
    }

    pub fn parent(&self) -> Option<&'run Self> {
        self.parent
    }
}
