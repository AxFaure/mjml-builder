use crate::model::{
    render_css_class, render_font_family, render_generic_attribute, render_padding_opt, Alignment,
    BaseComponent, Color, CssClass, FontStyle, Html, MjBaseComponent, MjBaseComponentAttributes,
    MjBaseComponentSimplified, MjBodyComponent, Padding, PxOrEm, PxOrPercent, SizePx,
    TextDecoration, Url, VerticalAlignment,
};
use std::fmt::{Display, Formatter};
use std::io::Write;

pub struct MjSocial {
    pub elements: Vec<MjSocialElement>,
    pub attributes: MjSocialAttributes,
}

pub struct MjSocialElement {
    pub content: Html,
    pub attributes: MjSocialElementAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialMode {
    Vertical,
    Horizontal,
}
impl Display for SocialMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "vertical"),
            Self::Horizontal => write!(f, "horizontal"),
        }
    }
}

#[derive(Default)]
pub struct MjSocialAttributes {
    pub align: Option<Alignment>,
    pub border_radius: Option<SizePx>,
    pub color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub container_background_color: Option<Color>,
    pub font_family: Vec<String>,
    pub font_size: Option<PxOrEm>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<u32>,
    pub icon_height: Option<PxOrPercent>,
    pub icon_size: Option<PxOrPercent>,
    pub inner_padding: Option<Padding>,
    pub line_height: Option<PxOrPercent>,
    pub mode: Option<SocialMode>,
    pub padding: Option<Padding>,
    pub icon_padding: Option<Padding>,
    pub text_padding: Option<Padding>,
    pub text_decoration: Option<TextDecoration>,
}

#[derive(Default)]
pub struct MjSocialElementAttributes {
    pub align: Option<Alignment>,
    pub alt: Option<String>,
    pub background_color: Option<Color>,
    pub border_radius: Option<SizePx>,
    pub color: Option<Color>,
    pub css_class: Vec<CssClass>,
    pub font_family: Vec<String>,
    pub font_size: Option<PxOrEm>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<u32>, // todo
    pub href: Option<Url>,
    pub icon_height: Option<PxOrPercent>,
    pub icon_size: Option<PxOrPercent>,
    pub line_height: Option<PxOrPercent>,
    pub name: Option<String>, //todo
    pub padding: Option<Padding>,
    pub icon_padding: Option<Padding>,
    pub text_padding: Option<Padding>,
    pub sizes: Option<String>, // todo
    pub src: Option<Url>,
    pub srcset: Option<String>, // todo
    pub rel: Option<String>,
    pub target: Option<String>,
    pub title: Option<String>,
    pub text_decoration: Option<TextDecoration>,
    pub vertical_align: Option<VerticalAlignment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialName {
    Facebook,
    Twitter,
    Google,
    Pinterest,
    Linkedin,
    Tumblr,
    Xing,
    FacebookNoShare,
    TwitterNoShare,
    GoogleNoShare,
    PinterestNoShare,
    LinkedinNoShare,
    TumblrNoShare,
    XingNoShare,
    Github,
    Instagram,
    Web,
    Snapchat,
    Youtube,
    Vimeo,
    Medium,
    Soundcloud,
    Dribble,
}
impl Display for SocialName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Facebook => write!(f, "facebook"),
            Self::Twitter => write!(f, "twitter"),
            Self::Google => write!(f, "google"),
            Self::Pinterest => write!(f, "pinterest"),
            Self::Linkedin => write!(f, "linkedin"),
            Self::Tumblr => write!(f, "tumblr"),
            Self::Xing => write!(f, "xing"),
            Self::FacebookNoShare => write!(f, "facebook-noshare"),
            Self::TwitterNoShare => write!(f, "twitter-noshare"),
            Self::GoogleNoShare => write!(f, "googleN-noshare"),
            Self::PinterestNoShare => write!(f, "pinterest-noshare"),
            Self::LinkedinNoShare => write!(f, "linkedin-noshare"),
            Self::TumblrNoShare => write!(f, "tumblr-noshare"),
            Self::XingNoShare => write!(f, "xing-noshare"),
            Self::Github => write!(f, "github"),
            Self::Instagram => write!(f, "instagram"),
            Self::Web => write!(f, "web"),
            Self::Snapchat => write!(f, "snapchat"),
            Self::Youtube => write!(f, "youtube"),
            Self::Vimeo => write!(f, "vimeo"),
            Self::Medium => write!(f, "medium"),
            Self::Soundcloud => write!(f, "soundcloud"),
            Self::Dribble => write!(f, "dribble"),
        }
    }
}

impl MjBodyComponent for MjSocial {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBaseComponentSimplified for MjSocial {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for element in &self.elements {
            Box::new(element).render(writer, depth)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-social"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}
impl MjBaseComponentSimplified for MjSocialElement {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-social-element"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjSocialAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
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
        render_generic_attribute(writer, "icon-height", &self.icon_height)?;
        render_generic_attribute(writer, "icon-size", &self.icon_size)?;
        render_padding_opt(writer, "inner-padding", &self.inner_padding)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_generic_attribute(writer, "mode", &self.mode)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_padding_opt(writer, "icon-padding", &self.icon_padding)?;
        render_padding_opt(writer, "text-padding", &self.text_padding)?;
        render_generic_attribute(writer, "text-decoration", &self.text_decoration)?;

        Ok(())
    }
}
impl MjBaseComponentAttributes for MjSocialElementAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "align", &self.align)?;
        render_generic_attribute(writer, "alt", &self.alt)?;
        render_generic_attribute(writer, "background-color", &self.background_color)?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_generic_attribute(writer, "color", &self.color)?;
        render_css_class(writer, &self.css_class)?;
        render_font_family(writer, &self.font_family)?;
        render_generic_attribute(writer, "font-size", &self.font_size)?;
        render_generic_attribute(writer, "font-style", &self.font_style)?;
        render_generic_attribute(writer, "font-weight", &self.font_weight)?;
        render_generic_attribute(writer, "href", &self.href)?;
        render_generic_attribute(writer, "icon-height", &self.icon_height)?;
        render_generic_attribute(writer, "icon-size", &self.icon_size)?;
        render_generic_attribute(writer, "line-height", &self.line_height)?;
        render_generic_attribute(writer, "name", &self.name)?;
        render_padding_opt(writer, "padding", &self.padding)?;
        render_padding_opt(writer, "icon-padding", &self.icon_padding)?;
        render_padding_opt(writer, "text-padding", &self.text_padding)?;
        render_generic_attribute(writer, "sizes", &self.sizes)?;
        render_generic_attribute(writer, "src", &self.src)?;
        render_generic_attribute(writer, "srcset", &self.srcset)?;
        render_generic_attribute(writer, "rel", &self.rel)?;
        render_generic_attribute(writer, "target", &self.target)?;
        render_generic_attribute(writer, "title", &self.title)?;
        render_generic_attribute(writer, "text-decoration", &self.text_decoration)?;
        render_generic_attribute(writer, "vertical-align", &self.vertical_align)?;
        Ok(())
    }
}
