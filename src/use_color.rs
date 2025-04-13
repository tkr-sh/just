use super::*;

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum UseColor {
    Auto,
    Always,
    Never,
}
