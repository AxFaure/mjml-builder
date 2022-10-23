use crate::model::{
    render_css_class, render_generic_attribute, render_padding_opt, BaseComponent, Color, CssClass,
    MjBaseComponentAttributes, MjBaseComponentSimplified, MjBodyComponent, Padding, SizePx,
};
use std::io::Write;

pub struct MjSpacer {
    pub attributes: MjSpacerAttributes,
}

#[derive(Default)]
pub struct MjSpacerAttributes {
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub padding: Option<Padding>,
    pub height: Option<SizePx>,
}

impl MjBodyComponent for MjSpacer {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjSpacer {
    fn render_content(&self, _writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-spacer"
    }

    fn has_content(&self) -> bool {
        false
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjSpacerAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "height", &self.height)?;
        Ok(())
    }
}
