use crate::model::{
    render_css_class, render_generic_attribute, render_padding_opt, Alignment, BaseComponent,
    BorderStyle, Color, CssClass, MjBaseComponentAttributes, MjBaseComponentSimplified,
    MjBodyComponent, Padding, PxOrPercent, SizePx,
};
use std::io::Write;

pub struct MjDivider {
    pub attributes: MjDividerAttributes,
}

#[derive(Default)]
pub struct MjDividerAttributes {
    pub border_color: Option<Color>,
    pub border_style: Option<BorderStyle>,
    pub border_width: Option<SizePx>,
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub padding: Option<Padding>,
    pub width: Option<PxOrPercent>,
    pub align: Option<Alignment>,
}

impl MjBodyComponent for MjDivider {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjDivider {
    fn render_content(&self, _writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-divider"
    }

    fn has_content(&self) -> bool {
        false
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjDividerAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "border-color", &self.border_color)?;
        render_generic_attribute(writer, "border-style", &self.border_style)?;
        render_generic_attribute(writer, "border-width", &self.border_width)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "width", &self.width)?;
        render_generic_attribute(writer, "align", &self.align)?;
        Ok(())
    }
}
