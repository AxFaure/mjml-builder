use crate::model::{
    render_css_class, render_generic_attribute, Alignment, BaseComponent, BorderProperties, Color,
    CssClass, Html, MjBaseComponent, MjBaseComponentAttributes, MjBaseComponentSimplified,
    MjBodyComponent, SizePx, Url,
};
use std::fmt::{Display, Formatter};
use std::io::Write;

pub struct MjCarousel {
    pub images: Vec<MjCarouselImage>,
    pub attributes: MjCarouselAttributes,
}

pub struct MjCarouselImage {
    pub content: Html,
    pub attributes: MjCarouselImageAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
}
impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
        }
    }
}

#[derive(Default)]
pub struct MjCarouselAttributes {
    pub align: Option<Alignment>,
    pub container_background_color: Option<Color>,
    pub border_radius: Option<SizePx>,
    pub css_class: Vec<CssClass>,
    pub icon_width: Option<SizePx>,
    pub left_icon: Option<Url>,
    pub right_icon: Option<Url>,
    pub tb_border: Option<BorderProperties>,
    pub tb_border_radius: Option<SizePx>,
    pub tb_border_hover_color: Option<Color>,
    pub tb_selected_border_color: Option<Color>,
    pub tb_width: Option<SizePx>,
    pub thumbnails: Option<Visibility>,
}

#[derive(Default)]
pub struct MjCarouselImageAttributes {
    pub alt: Option<String>,
    pub css_class: Vec<CssClass>,
    pub href: Option<Url>,
    pub rel: Option<String>,
    pub src: Option<Url>,
    pub target: Option<String>,
    pub thumbnail_src: Option<String>,
    pub title: Option<String>,
}

impl MjBodyComponent for MjCarousel {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjCarouselImage {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-carousel-image"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}
impl MjBaseComponentSimplified for MjCarousel {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for image in &self.images {
            Box::new(image).render(writer, depth)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-carousel"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjCarouselAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(
            writer,
            "container-background-color",
            &self.container_background_color,
        )?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_css_class(writer, &self.css_class)?;
        render_generic_attribute(writer, "icon-width", &self.icon_width)?;
        render_generic_attribute(writer, "left-icon", &self.left_icon)?;
        render_generic_attribute(writer, "right-icon", &self.right_icon)?;
        render_generic_attribute(writer, "tb-border", &self.tb_border)?;
        render_generic_attribute(writer, "tb-border-radius", &self.tb_border_radius)?;
        render_generic_attribute(writer, "tb-border-hover-color", &self.tb_border_hover_color)?;
        render_generic_attribute(
            writer,
            "tb-selected-border-color",
            &self.tb_selected_border_color,
        )?;
        render_generic_attribute(writer, "tb-width", &self.tb_width)?;
        render_generic_attribute(writer, "thumbnails", &self.thumbnails)?;
        Ok(())
    }
}
impl MjBaseComponentAttributes for MjCarouselImageAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "alt", &self.alt)?;
        render_css_class(writer, &self.css_class)?;
        render_generic_attribute(writer, "href", &self.href)?;
        render_generic_attribute(writer, "rel", &self.rel)?;
        render_generic_attribute(writer, "src", &self.src)?;
        render_generic_attribute(writer, "target", &self.target)?;
        render_generic_attribute(writer, "thumbnail-src", &self.thumbnail_src)?;
        render_generic_attribute(writer, "title", &self.title)?;
        Ok(())
    }
}
