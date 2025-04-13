use super::*;

pub struct RecipeSignature<'a> {
    pub name: &'a str,
    pub recipe: &'a Recipe<'a>,
}

impl<'a> ColorDisplay for RecipeSignature<'a> {
    fn fmt(&self, f: &mut Formatter, color: Color) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for parameter in &self.recipe.parameters {
            write!(f, " {}", parameter.color_display(color))?;
        }
        Ok(())
    }
}
