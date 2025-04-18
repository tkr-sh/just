use super::*;

#[derive(Debug, Clone)]
pub enum Setting<'src> {
    AllowDuplicateRecipes(bool),
    AllowDuplicateVariables(bool),
    DotenvFilename(StringLiteral<'src>),
    DotenvLoad(bool),
    DotenvPath(StringLiteral<'src>),
    DotenvRequired(bool),
    Export(bool),
    Fallback(bool),
    IgnoreComments(bool),
    PositionalArguments(bool),
    Quiet(bool),
    ScriptInterpreter(Interpreter<'src>),
    Shell(Interpreter<'src>),
    Tempdir(StringLiteral<'src>),
    Unstable(bool),
    WindowsPowerShell(bool),
    WindowsShell(Interpreter<'src>),
    WorkingDirectory(StringLiteral<'src>),
}

impl<'src> Display for Setting<'src> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::AllowDuplicateRecipes(value) |
            Self::AllowDuplicateVariables(value) |
            Self::DotenvLoad(value) |
            Self::DotenvRequired(value) |
            Self::Export(value) |
            Self::Fallback(value) |
            Self::IgnoreComments(value) |
            Self::PositionalArguments(value) |
            Self::Quiet(value) |
            Self::Unstable(value) |
            Self::WindowsPowerShell(value) => write!(f, "{value}"),
            Self::ScriptInterpreter(shell) | Self::Shell(shell) | Self::WindowsShell(shell) => {
                write!(f, "[{shell}]")
            },
            Self::DotenvFilename(value) |
            Self::DotenvPath(value) |
            Self::Tempdir(value) |
            Self::WorkingDirectory(value) => {
                write!(f, "{value}")
            },
        }
    }
}
