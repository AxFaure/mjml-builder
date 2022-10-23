use crate::model::{
    render_css_class, render_generic_attribute, render_padding_opt, BaseComponent,
    BorderProperties, Color, CssClass, Direction, MjBaseComponent, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyColumnComponent, MjBodyComponent, MjBodySectionComponent,
    Padding, SectionCompatible, SizePx, TextAlignment, Url,
};
use std::io::Write;

pub struct MjSection {
    pub attributes: MjSectionAttributes,
    pub content: Vec<Box<dyn MjBodyColumnComponent>>,
}

#[derive(Default)]
pub struct MjSectionAttributes {
    pub background_color: Option<Color>,
    pub background_position_x: Option<String>, // todo
    pub background_position_y: Option<String>, // todo
    pub background_repeat: Option<String>,     // todo
    pub background_size: Option<String>,       // todo
    pub background_url: Option<Url>,
    pub border: Option<BorderProperties>,
    pub border_bottom: Option<BorderProperties>,
    pub border_left: Option<BorderProperties>,
    pub border_radius: Option<SizePx>,
    pub border_right: Option<BorderProperties>,
    pub border_top: Option<BorderProperties>,
    pub css_class: Vec<CssClass>,
    pub direction: Option<Direction>,
    pub full_width: bool,
    pub padding: Option<Padding>,
    pub text_align: Option<TextAlignment>, // todo
}

impl MjBodyComponent for MjSection {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjSection {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for column in &self.content {
            column.to_column().render(writer, depth + 1)?
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-section"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}
impl MjBodySectionComponent for MjSection {
    fn to_section(&self) -> SectionCompatible {
        SectionCompatible::Section(self)
    }
}

impl MjBaseComponentAttributes for MjSectionAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "background-position-x", &self.background_position_x)?;
        render_generic_attribute(writer, "background-position-y", &self.background_position_y)?;
        render_generic_attribute(writer, "background-repeat", &self.background_repeat)?;
        render_generic_attribute(writer, "background-size", &self.background_size)?;
        render_generic_attribute(writer, "background-url", &self.background_url)?;
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "border-bottom", &self.border_bottom)?;
        render_generic_attribute(writer, "border-left", &self.border_left)?;
        render_generic_attribute(writer, "border-right", &self.border_right)?;
        render_generic_attribute(writer, "border-top", &self.border_top)?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_css_class(writer, &self.css_class)?;
        render_generic_attribute(writer, "direction", &self.direction)?;
        if self.full_width {
            write!(writer, " full-width=\"full-width\"")?;
        }
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "text-align", &self.text_align)?;
        Ok(())
    }
}
