use crate::model::{
    render_css_class, render_generic_attribute, Color, ColumnCompatible, CssClass, Direction,
    MjBaseComponent, MjBaseComponentAttributes, MjBaseComponentSimplified, MjBodyColumnComponent,
    PxOrPercent, VerticalAlignment,
};
use std::io::Write;

pub struct MjGroup {
    // columns with a width in percent
    pub columns: Vec<Box<dyn MjBodyColumnComponent>>,
    pub attributes: MjGroupAttributes,
}

#[derive(Default)]
pub struct MjGroupAttributes {
    pub width: Option<PxOrPercent>,
    pub vertical_align: Option<VerticalAlignment>,
    pub background_color: Option<Color>,
    pub direction: Option<Direction>,
    pub css_class: Vec<CssClass>,
}

impl MjBodyColumnComponent for MjGroup {
    fn to_column(&self) -> ColumnCompatible {
        ColumnCompatible::Group(self)
    }
}
impl MjBaseComponentSimplified for MjGroup {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for column in &self.columns {
            Box::new(column.to_column()).render(writer, depth + 1)?
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-group"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjGroupAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "width", &self.width)?;
        render_generic_attribute(writer, "vertical-align", &self.vertical_align)?;
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "direction", &self.direction)?;
        render_css_class(writer, &self.css_class)?;
        Ok(())
    }
}
