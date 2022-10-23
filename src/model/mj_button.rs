use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, render_padding_opt, Alignment,
    BaseComponent, BorderProperties, Color, CssClass, FontStyle, Html, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyComponent, Padding, PxOrEm, SizePx, TextDecoration,
    TextTransform, Url, VerticalAlignment,
};
use std::io::Write;

pub struct MjButton {
    pub content: Html,
    pub attributes: MjButtonAttributes,
}

#[derive(Default)]
pub struct MjButtonAttributes {
    pub align: Option<Alignment>,
    pub background_color: Option<Color>,
    pub border: Option<BorderProperties>,
    pub border_bottom: Option<BorderProperties>,
    pub border_left: Option<BorderProperties>,
    pub border_radius: Option<SizePx>,
    pub border_right: Option<BorderProperties>,
    pub border_top: Option<BorderProperties>,
    pub color: Option<Color>,
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<u32>,
    pub height: Option<SizePx>,
    pub href: Option<Url>,
    pub inner_padding: Option<Padding>,
    pub letter_spacing: Option<PxOrEm>,
    pub line_height: Option<String>,
    pub padding: Option<Padding>,
    pub rel: Option<String>,
    pub target: Option<String>,
    pub text_align: Option<String>,
    pub text_decoration: Option<TextDecoration>,
    pub text_transform: Option<TextTransform>,
    pub title: Option<String>,
    pub vertical_align: Option<VerticalAlignment>,
    pub width: Option<SizePx>,
}

impl MjBodyComponent for MjButton {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}

impl MjBaseComponentSimplified for MjButton {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-button"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjButtonAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "border-bottom", &self.border_bottom)?;
        render_generic_attribute(writer, "border-left", &self.border_left)?;
        render_generic_attribute(writer, "border-right", &self.border_right)?;
        render_generic_attribute(writer, "border-top", &self.border_top)?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_generic_attribute(writer, "color", &self.color)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "font-style", &self.font_style)?;
        render_generic_attribute(writer, "font-weight", &self.font_weight)?;
        render_generic_attribute(writer, "height", &self.height)?;
        render_generic_attribute(writer, "href", &self.href)?;
        render_padding_opt(writer, "inner-padding", &self.inner_padding)?;
        render_generic_attribute(writer, "letter-spacing", &self.letter_spacing)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "rel", &self.rel)?;
        render_generic_attribute(writer, "target", &self.target)?;
        render_generic_attribute(writer, "text-align", &self.text_align)?;
        render_generic_attribute(writer, "text-decoration", &self.text_decoration)?;
        render_generic_attribute(writer, "text-transform", &self.text_transform)?;
        render_generic_attribute(writer, "title", &self.title)?;
        render_generic_attribute(writer, "vertical-align", &self.vertical_align)?;
        render_generic_attribute(writer, "width", &self.width)?;
        Ok(())
    }
}
