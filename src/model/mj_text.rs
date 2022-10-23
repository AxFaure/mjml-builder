use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, render_padding_opt,
    BaseComponent, Color, CssClass, FontStyle, Html, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyComponent, Padding, PxOrEm, SizePx, TextAlignment,
    TextDecoration, TextTransform,
};
use std::io::Write;

pub struct MjText {
    pub content: Html,
    pub attributes: MjTextAttributes,
}

#[derive(Default)]
pub struct MjTextAttributes {
    pub color: Option<Color>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<u32>, // todo
    pub line_height: Option<SizePx>,
    pub letter_spacing: Option<PxOrEm>,
    pub height: Option<SizePx>,
    pub text_decoration: Option<TextDecoration>,
    pub text_transform: Option<TextTransform>,
    pub align: Option<TextAlignment>,
    pub container_background_color: Option<Color>,
    pub padding: Option<Padding>,
    pub css_class: Vec<CssClass>,
}

impl MjBodyComponent for MjText {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjText {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-text"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjTextAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "color", &self.color)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "font-style", &self.font_style)?;
        render_generic_attribute(writer, "font-weight", &self.font_weight)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_generic_attribute(writer, "letter-spacing", &self.letter_spacing)?;
        render_generic_attribute(writer, "height", &self.height)?;
        render_generic_attribute(writer, "text-decoration", &self.text_decoration)?;
        render_generic_attribute(writer, "text-transform", &self.text_transform)?;
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_css_class(writer, &self.css_class)?;
        Ok(())
    }
}
