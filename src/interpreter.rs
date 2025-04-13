use super::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Interpreter<'src> {
    pub arguments: Vec<StringLiteral<'src>>,
    pub command: StringLiteral<'src>,
}

impl<'src> Interpreter<'src> {
    pub fn default_script_interpreter() -> &'static Interpreter<'static> {
        static INSTANCE: Lazy<Interpreter<'static>> = Lazy::new(|| {
            Interpreter {
                arguments: vec![StringLiteral::from_raw("-eu")],
                command: StringLiteral::from_raw("sh"),
            }
        });
        &INSTANCE
    }
}

impl<'src> Display for Interpreter<'src> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.command)?;

        for argument in &self.arguments {
            write!(f, ", {argument}")?;
        }

        Ok(())
    }
}
