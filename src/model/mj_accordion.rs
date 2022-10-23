use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, Alignment, BaseComponent,
    BorderProperties, Color, CssClass, Html, MjBaseComponentAttributes, MjBaseComponentSimplified,
    MjBodyComponent, Padding, PxOrEm, SizePx, Url,
};
use crate::model::{render_padding_opt, MjBaseComponent};
use std::fmt::{Display, Formatter};
use std::io::Write;

pub struct MjAccordion {
    pub elements: Vec<MjAccordionElement>,
    pub attributes: MjAccordionAttributes,
}

pub struct MjAccordionElement {
    pub title: MjAccordionElementTitle,
    pub text: MjAccordionElementText,

    pub attributes: MjAccordionElementAttributes,
}

pub struct MjAccordionElementTitle {
    pub content: Html,

    pub attributes: MjAccordionElementTitleAttributes,
}

pub struct MjAccordionElementText {
    pub content: Html,

    pub attributes: MjAccordionElementTextAttributes,
}

#[derive(Default)]
pub struct MjAccordionElementTitleAttributes {
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub padding: Option<Padding>,
}

#[derive(Default)]
pub struct MjAccordionElementTextAttributes {
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub font_weight: Option<u32>,
    pub letter_spacing: Option<PxOrEm>,
    pub line_height: Option<SizePx>,
    pub padding: Option<Padding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPosition {
    Left,
    Right,
}
impl Display for IconPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

#[derive(Default)]
pub struct MjAccordionElementAttributes {
    pub background_color: Option<Color>,
    pub border: Option<BorderProperties>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub icon_align: Option<Alignment>,
    pub icon_height: Option<SizePx>,
    pub icon_position: Option<IconPosition>,
    pub icon_unwrapped_alt: Option<String>,
    pub icon_unwrapped_url: Option<Url>,
    pub icon_width: Option<SizePx>,
    pub icon_wrapped_alt: Option<String>,
    pub icon_wrapped_url: Option<Url>,
}

#[derive(Default)]
pub struct MjAccordionAttributes {
    pub border: Option<BorderProperties>,
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub icon_align: Option<Alignment>,
    pub icon_height: Option<SizePx>,
    pub icon_position: Option<IconPosition>,
    pub icon_unwrapped_alt: Option<String>,
    pub icon_unwrapped_url: Option<Url>,
    pub icon_width: Option<SizePx>,
    pub icon_wrapped_alt: Option<String>,
    pub icon_wrapped_url: Option<Url>,

    pub padding: Option<Padding>,
}

impl MjBodyComponent for MjAccordion {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}

impl MjBaseComponentSimplified for MjAccordion {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for element in &self.elements {
            Box::new(element).render(writer, depth + 1)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-accordion"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjAccordionAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "icon-align", &self.icon_align)?;
        render_generic_attribute(writer, "icon-height", &self.icon_height)?;
        render_generic_attribute(writer, "icon-position", &self.icon_position)?;
        render_generic_attribute(writer, "icon-unwrapped-alt", &self.icon_unwrapped_alt)?;
        render_generic_attribute(writer, "icon-unwrapped-url", &self.icon_unwrapped_url)?;
        render_generic_attribute(writer, "icon-width", &self.icon_width)?;
        render_generic_attribute(writer, "icon-wrapped-alt", &self.icon_wrapped_alt)?;
        render_generic_attribute(writer, "icon-wrapped-url", &self.icon_wrapped_url)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        Ok(())
    }
}

impl MjBaseComponentSimplified for MjAccordionElement {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        self.title.render(writer, depth)?;
        self.text.render(writer, depth)?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-accordion-element"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjAccordionElementAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "icon-align", &self.icon_align)?;
        render_generic_attribute(writer, "icon-height", &self.icon_height)?;
        render_generic_attribute(writer, "icon-position", &self.icon_position)?;
        render_generic_attribute(writer, "icon-unwrapped-alt", &self.icon_unwrapped_alt)?;
        render_generic_attribute(writer, "icon-unwrapped-url", &self.icon_unwrapped_url)?;
        render_generic_attribute(writer, "icon-width", &self.icon_width)?;
        render_generic_attribute(writer, "icon-wrapped-alt", &self.icon_wrapped_alt)?;
        render_generic_attribute(writer, "icon-wrapped-url", &self.icon_wrapped_url)?;
        Ok(())
    }
}

impl MjBaseComponentSimplified for MjAccordionElementTitle {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-accordion-title"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjAccordionElementTitleAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "color", &self.color)?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        Ok(())
    }
}

impl MjBaseComponentSimplified for MjAccordionElementText {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-accordion-text"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjAccordionElementTextAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "color", &self.color)?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "font-weight", &self.font_weight)?;
        render_generic_attribute(writer, "letter-spacing", &self.letter_spacing)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        Ok(())
    }
}
