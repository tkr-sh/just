#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Verbosity {
  Quiet,
  Taciturn,
  Loquacious,
  Grandiloquent,
}

impl Verbosity {
  pub fn from_flag_occurrences(flag_occurrences: u8) -> Self {
    match flag_occurrences {
      0 => Self::Taciturn,
      1 => Self::Loquacious,
      _ => Self::Grandiloquent,
    }
  }

  pub fn quiet(self) -> bool {
    self == Self::Quiet
  }

  pub fn loud(self) -> bool {
    !self.quiet()
  }

  pub fn loquacious(self) -> bool {
    self >= Self::Loquacious
  }

  pub fn grandiloquent(self) -> bool {
    self >= Self::Grandiloquent
  }

  pub const fn default() -> Self {
    Self::Taciturn
  }
}

impl Default for Verbosity {
  fn default() -> Self {
    Self::default()
  }
}
