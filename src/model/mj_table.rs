use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, render_padding_opt, Alignment,
    BaseComponent, BorderProperties, Color, CssClass, Html, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyComponent, Padding, PxOrPercent, SizePx,
};
use std::fmt::{Display, Formatter};
use std::io::Write;

pub struct MjTable {
    // todo: should be restricted to what can be used inside an html table
    pub content: Html,
    pub attributes: MjTableAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableLayout {
    Auto,
    Fixed,
    Initial,
    Inherit,
}
impl Display for TableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Fixed => write!(f, "fixed"),
            Self::Initial => write!(f, "initial"),
            Self::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Default)]
pub struct MjTableAttributes {
    pub align: Option<Alignment>,
    pub border: Option<BorderProperties>,
    pub cellpadding: Option<SizePx>,
    pub cellspacing: Option<SizePx>,
    pub color: Option<Color>,
    pub container_background_color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub line_height: Option<PxOrPercent>,
    pub padding: Option<Padding>,
    pub presentation_role: bool,
    pub table_layout: Option<TableLayout>,
    pub width: Option<PxOrPercent>,
}

impl MjBodyComponent for MjTable {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjTable {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-table"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjTableAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "cellpadding", &self.cellpadding)?;
        render_generic_attribute(writer, "cellspacing", &self.cellspacing)?;
        render_generic_attribute(writer, "color", &self.color)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        if self.presentation_role {
            write!(writer, " role=\"presentation\"")?;
        }
        render_generic_attribute(writer, "table-layout", &self.table_layout)?;
        render_generic_attribute(writer, "width", &self.width)?;
        Ok(())
    }
}
