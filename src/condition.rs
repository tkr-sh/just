use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Condition<'src> {
  pub lhs: Box<Expression<'src>>,
  pub rhs: Box<Expression<'src>>,
  pub operator: ConditionalOperator,
}

impl<'src> Display for Condition<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
  }
}

impl<'src> Serialize for Condition<'src> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(None)?;
    seq.serialize_element(&self.operator.to_string())?;
    seq.serialize_element(&self.lhs)?;
    seq.serialize_element(&self.rhs)?;
    seq.end()
  }
}
