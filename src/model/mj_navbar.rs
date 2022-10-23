use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, render_padding_opt, Alignment,
    BaseComponent, Color, CssClass, FontStyle, Html, MjBaseComponent, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyComponent, Padding, PxOrEm, SizePx, TextDecoration,
    TextTransform, Url,
};
use std::io::Write;

pub struct MjNavbar {
    pub links: Vec<MjNavbarLink>,
    pub attributes: MjNavbarAttributes,
}

pub struct MjNavbarLink {
    pub content: Html,
    pub attributes: MjNavbarLinkAttributes,
}

#[derive(Default)]
pub struct MjNavbarAttributes {
    pub align: Option<Alignment>,
    pub base_url: Option<String>,
    pub css_class: Vec<CssClass>,
    pub hamburger: Option<MjNavbarHamburgerAttributes>,
}
#[derive(Default)]
pub struct MjNavbarHamburgerAttributes {
    pub ico_align: Option<Alignment>,
    pub ico_close: Option<u32>, // todo
    pub ico_color: Option<Color>,
    pub ico_font_family: Vec<String>,
    pub ico_font_size: Option<SizePx>,
    pub ico_line_height: Option<SizePx>,
    pub ico_open: Option<u32>, // todo
    pub ico_padding: Option<Padding>,
    pub ico_text_decoration: Option<TextDecoration>,
    pub ico_text_transform: Option<TextTransform>,
}

#[derive(Default)]
pub struct MjNavbarLinkAttributes {
    pub color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<SizePx>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<u32>,
    pub href: Option<Url>, // todo
    pub letter_spacing: Option<PxOrEm>,
    pub line_height: Option<SizePx>,
    pub padding: Option<Padding>,
    pub rel: Option<String>,
    pub target: Option<String>,
    pub text_decoration: Option<TextDecoration>,
    pub text_transform: Option<TextTransform>,
}

impl MjBodyComponent for MjNavbar {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjNavbar {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for link in &self.links {
            Box::new(link).render(writer, depth)?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-navbar-link"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}
impl MjBaseComponentSimplified for MjNavbarLink {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-navbar-link"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjNavbarAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(writer, "base_url", &self.base_url)?;
        render_css_class(writer, &self.css_class)?;
        if let Some(hamburger) = &self.hamburger {
            Box::new(hamburger).render(writer)?;
        }
        Ok(())
    }
}
impl MjBaseComponentAttributes for MjNavbarHamburgerAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "ico-align", &self.ico_align)?;
        render_generic_attribute(writer, "ico-close", &self.ico_close)?;
        render_generic_attribute(writer, "ico-color", &self.ico_color)?;
        if !self.ico_font_family.is_empty() {
            let families = self.ico_font_family.join(", ");
            write!(writer, " ico-font-family=\"{}\"", families)?;
        }
        render_generic_attribute(writer, "ico-font-size", &self.ico_font_size)?;
        render_generic_attribute(writer, "ico-line-height", &self.ico_line_height)?;
        render_generic_attribute(writer, "ico-open", &self.ico_open)?;
        render_padding_opt(writer, "ico-padding", &self.ico_padding)?;
        render_generic_attribute(writer, "ico-text-decoration", &self.ico_text_decoration)?;
        render_generic_attribute(writer, "ico-text-transform", &self.ico_text_transform)?;
        Ok(())
    }
}
impl MjBaseComponentAttributes for MjNavbarLinkAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "color", &self.color)?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "font-style", &self.font_style)?;
        render_generic_attribute(writer, "font-weight", &self.font_weight)?;
        render_generic_attribute(writer, "href", &self.href)?;
        render_generic_attribute(writer, "letter-spacing", &self.letter_spacing)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "rel", &self.rel)?;
        render_generic_attribute(writer, "target", &self.target)?;
        render_generic_attribute(writer, "text-decoration", &self.text_decoration)?;
        render_generic_attribute(writer, "text-transform", &self.text_transform)?;
        Ok(())
    }
}
