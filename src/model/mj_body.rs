use crate::model::{
    render_css_class, render_generic_attribute, Color, CssClass, MjBaseComponent,
    MjBaseComponentAttributes, MjBaseComponentSimplified, MjBodySectionComponent, SizePx,
};
use std::io::Write;

pub struct MjBody {
    pub content: Vec<Box<dyn MjBodySectionComponent>>,
    pub attributes: MjBodyAttributes,
}

pub struct MjBodyAttributes {
    pub background_color: Option<Color>,
    pub width: SizePx,
    pub css_class: Vec<CssClass>,
}

impl Default for MjBodyAttributes {
    fn default() -> Self {
        Self {
            background_color: None,
            width: SizePx::new(600),
            css_class: vec![],
        }
    }
}

impl MjBaseComponentSimplified for MjBody {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for section in &self.content {
            let section = section.to_section();
            (Box::new(section)).render(writer, depth + 1)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-body"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjBodyAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_css_class(writer, &self.css_class)?;
        write!(writer, " width=\"{}\"", self.width)?;

        Ok(())
    }
}
