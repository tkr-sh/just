#[derive(Debug, PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
pub enum StringDelimiter {
    Backtick,
    QuoteDouble,
    QuoteSingle,
}
