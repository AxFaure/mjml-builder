use crate::model::{
    render_css_class, render_generic_attribute, render_padding_opt, Alignment, BaseComponent,
    BorderProperties, Color, CssClass, MjBaseComponentAttributes, MjBaseComponentSimplified,
    MjBodyComponent, Padding, SizePx, Url,
};
use std::io::Write;

pub struct MjImage {
    pub attributes: MjImageAttributes,
}

#[derive(Default)]
pub struct MjImageAttributes {
    pub align: Option<Alignment>,
    pub alt: Option<String>,
    pub border: Option<BorderProperties>,
    pub border_bottom: Option<BorderProperties>,
    pub border_left: Option<BorderProperties>,
    pub border_radius: Option<SizePx>,
    pub border_right: Option<BorderProperties>,
    pub border_top: Option<BorderProperties>,
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub fluid_on_mobile: Option<bool>,
    pub height: Option<SizePx>,
    pub href: Option<Url>,
    pub name: Option<String>,
    pub padding: Option<Padding>,
    pub rel: Option<String>,
    pub sizes: Option<String>, // todo ??
    pub src: Url,
    pub srcset: Option<String>, // todo ??
    pub target: Option<String>,
    pub title: Option<String>,
    pub usemap: Option<String>,
    pub width: Option<SizePx>,
}

impl MjBodyComponent for MjImage {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjImage {
    fn render_content(&self, _writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-image"
    }

    fn has_content(&self) -> bool {
        false
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjImageAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(writer, "alt", &self.alt)?;
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "border-bottom", &self.border_bottom)?;
        render_generic_attribute(writer, "border-left", &self.border_left)?;
        render_generic_attribute(writer, "border-right", &self.border_right)?;
        render_generic_attribute(writer, "border-top", &self.border_top)?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_generic_attribute(writer, "fluid-on-mobile", &self.fluid_on_mobile)?;
        render_generic_attribute(writer, "height", &self.height)?;
        render_generic_attribute(writer, "href", &self.href)?;
        render_generic_attribute(writer, "name", &self.name)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "rel", &self.rel)?;
        render_generic_attribute(writer, "sizes", &self.sizes)?;
        write!(writer, " src=\"{}\"", self.src)?;
        render_generic_attribute(writer, "srcset", &self.srcset)?;
        render_generic_attribute(writer, "target", &self.target)?;
        render_generic_attribute(writer, "title", &self.title)?;
        render_generic_attribute(writer, "usemap", &self.usemap)?;
        render_generic_attribute(writer, "width", &self.width)?;
        Ok(())
    }
}
