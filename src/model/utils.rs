use std::fmt::{Display, Formatter};
use std::io::Write;

pub(crate) fn build_indentation(depth: usize) -> String {
    "\t".repeat(depth)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Center => write!(f, "center"),
            Self::Right => write!(f, "right"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}
impl Display for TextAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Center => write!(f, "center"),
            Self::Right => write!(f, "right"),
            Self::Justify => write!(f, "justify"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
impl Display for VerticalAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Center => write!(f, "center"),
            Self::Bottom => write!(f, "bottom"),
        }
    }
}

pub type Color = hex_color::HexColor;
pub type Html = String; // todo
pub type CssClass = String;
pub type Css = String;
pub type Url = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizePx(u32);
impl SizePx {
    pub fn new(size: u32) -> Self {
        Self(size)
    }
}
impl Display for SizePx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 != 0 {
            write!(f, "{}px", self.0)
        } else {
            write!(f, "0")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PxOrEm {
    Px(SizePx),
    Em(f32),
}
impl Display for PxOrEm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(px) => write!(f, "{}", px),
            Self::Em(em) => write!(f, "{}em", em),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PxOrPercent {
    Px(SizePx),
    Percent(f32),
}
impl Display for PxOrPercent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(px) => write!(f, "{}", px),
            Self::Percent(percent) => write!(f, "{}%", percent),
        }
    }
}

pub struct Padding {
    pub top: PaddingValue,
    pub right: PaddingValue,
    pub bottom: PaddingValue,
    pub left: PaddingValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingValue {
    Px(SizePx),
    Inherit,
}
impl Display for PaddingValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(px) => write!(f, "{}", px),
            Self::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Dashed,
    Dotted,
    Solid,
}
impl Display for BorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BorderStyle::Dashed => write!(f, "dashed"),
            BorderStyle::Dotted => write!(f, "dotted"),
            BorderStyle::Solid => write!(f, "solid"),
        }
    }
}

pub struct BorderProperties {
    pub width: Option<SizePx>,
    pub style: Option<BorderStyle>,
    pub color: Option<Color>,
}

impl Display for BorderProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.width.is_none() && self.style.is_none() && self.color.is_none() {
            return write!(f, "none");
        }

        if let Some(style) = self.style {
            write!(f, "{}", style)?;
        }
        if let Some(width) = self.width {
            write!(f, "{}", width)?;
        }
        if let Some(color) = &self.color {
            write!(f, "{}", color)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}
impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Italic => write!(f, "italic"),
            Self::Oblique => write!(f, "oblique"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    None,
}
impl Display for TextDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Underline => write!(f, "underline"),
            Self::Overline => write!(f, "overline"),
            Self::LineThrough => write!(f, "lineThrough"),
            Self::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    Capitalize,
    Uppercase,
    Lowercase,
    None,
}
impl Display for TextTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capitalize => write!(f, "capitalize"),
            Self::Uppercase => write!(f, "uppercase"),
            Self::Lowercase => write!(f, "lowercase"),
            Self::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ltr,
    Rtl,
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ltr => write!(f, "ltr"),
            Self::Rtl => write!(f, "rtl"),
        }
    }
}

pub(crate) fn render_css_class(
    writer: &mut dyn Write,
    css_class: &[CssClass],
) -> std::io::Result<()> {
    if !css_class.is_empty() {
        let classes = css_class.join(", ");
        write!(writer, " css-class=\"{}\"", classes)?;
    }

    Ok(())
}
pub(crate) fn render_font_family(
    writer: &mut dyn Write,
    font_family: &[String],
) -> std::io::Result<()> {
    if !font_family.is_empty() {
        let families = font_family.join(", ");
        write!(writer, " font-family=\"{}\"", families)?;
    }

    Ok(())
}

pub(crate) fn render_generic_attribute<T: Display>(
    writer: &mut dyn Write,
    name: &str,
    value: &Option<T>,
) -> std::io::Result<()> {
    if let Some(value) = value {
        write!(writer, " {}=\"{}\"", name, value)?;
    }
    Ok(())
}

pub(crate) fn render_padding_opt(
    writer: &mut dyn Write,
    prefix: &str,
    padding: &Option<Padding>,
) -> std::io::Result<()> {
    if let Some(padding) = padding {
        render_padding(writer, prefix, padding)?;
    }
    Ok(())
}

pub(crate) fn render_padding(
    writer: &mut dyn Write,
    prefix: &str,
    padding: &Padding,
) -> std::io::Result<()> {
    write!(writer, " {}-bottom=\"{}\"", prefix, padding.bottom)?;
    write!(writer, " {}-top=\"{}\"", prefix, padding.top)?;
    write!(writer, " {}-left=\"{}\"", prefix, padding.left)?;
    write!(writer, " {}-right=\"{}\"", prefix, padding.right)?;
    Ok(())
}
