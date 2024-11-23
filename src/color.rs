use {
  super::*,
  ansi_term::{ANSIGenericString, Color::*, Prefix, Style, Suffix},
  std::io::{self, IsTerminal},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
  is_terminal: bool,
  style: Style,
  use_color: UseColor,
}

impl Color {
  fn restyle(self, style: Style) -> Self {
    Self { style, ..self }
  }

  fn redirect(self, stream: impl IsTerminal) -> Self {
    Self {
      is_terminal: stream.is_terminal(),
      ..self
    }
  }

  fn effective_style(&self) -> Style {
    if self.active() {
      self.style
    } else {
      Style::new()
    }
  }

  pub fn auto() -> Self {
    Self::default()
  }

  pub fn always() -> Self {
    Self {
      use_color: UseColor::Always,
      ..Self::default()
    }
  }

  pub fn never() -> Self {
    Self {
      use_color: UseColor::Never,
      ..Self::default()
    }
  }

  pub fn stderr(self) -> Self {
    self.redirect(io::stderr())
  }

  pub fn stdout(self) -> Self {
    self.redirect(io::stdout())
  }

  pub fn context(self) -> Self {
    self.restyle(Style::new().fg(Blue).bold())
  }

  pub fn doc(self) -> Self {
    self.restyle(Style::new().fg(Blue))
  }

  pub fn doc_backtick(self) -> Self {
    self.restyle(Style::new().fg(Cyan))
  }

  pub fn error(self) -> Self {
    self.restyle(Style::new().fg(Red).bold())
  }

  pub fn group(self) -> Self {
    self.restyle(Style::new().fg(Yellow).bold())
  }

  pub fn warning(self) -> Self {
    self.restyle(Style::new().fg(Yellow).bold())
  }

  pub fn banner(self) -> Self {
    self.restyle(Style::new().fg(Cyan).bold())
  }

  pub fn command(self, foreground: Option<ansi_term::Color>) -> Self {
    self.restyle(Style {
      foreground,
      is_bold: true,
      ..Style::default()
    })
  }

  pub fn parameter(self) -> Self {
    self.restyle(Style::new().fg(Cyan))
  }

  pub fn message(self) -> Self {
    self.restyle(Style::new().bold())
  }

  pub fn annotation(self) -> Self {
    self.restyle(Style::new().fg(Purple))
  }

  pub fn string(self) -> Self {
    self.restyle(Style::new().fg(Green))
  }

  pub fn diff_added(self) -> Self {
    self.restyle(Style::new().fg(Green))
  }

  pub fn diff_deleted(self) -> Self {
    self.restyle(Style::new().fg(Red))
  }

  pub fn active(&self) -> bool {
    match self.use_color {
      UseColor::Always => true,
      UseColor::Never => false,
      UseColor::Auto => self.is_terminal,
    }
  }

  pub fn paint<'a>(&self, text: &'a str) -> ANSIGenericString<'a, str> {
    self.effective_style().paint(text)
  }

  pub fn prefix(&self) -> Prefix {
    self.effective_style().prefix()
  }

  pub fn suffix(&self) -> Suffix {
    self.effective_style().suffix()
  }
}

impl From<UseColor> for Color {
  fn from(use_color: UseColor) -> Self {
    Self {
      use_color,
      ..Default::default()
    }
  }
}

impl Default for Color {
  fn default() -> Self {
    Self {
      is_terminal: false,
      style: Style::new(),
      use_color: UseColor::Auto,
    }
  }
}
