use super::*;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum DumpFormat {
    Json,
    Just,
}
