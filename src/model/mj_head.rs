use crate::model::mj_accordion::{
    MjAccordionAttributes, MjAccordionElementAttributes, MjAccordionElementTextAttributes,
    MjAccordionElementTitleAttributes,
};
use crate::model::mj_button::MjButtonAttributes;
use crate::model::mj_carousel::{MjCarouselAttributes, MjCarouselImageAttributes};
use crate::model::mj_column::MjColumnAttributes;
use crate::model::mj_divider::MjDividerAttributes;
use crate::model::mj_group::MjGroupAttributes;
use crate::model::mj_hero::MjHeroAttributes;
use crate::model::mj_image::MjImageAttributes;
use crate::model::mj_navbar::{MjNavbarAttributes, MjNavbarLinkAttributes};
use crate::model::mj_section::MjSectionAttributes;
use crate::model::mj_social::{MjSocialAttributes, MjSocialElementAttributes};
use crate::model::mj_spacer::MjSpacerAttributes;
use crate::model::mj_table::MjTableAttributes;
use crate::model::mj_text::MjTextAttributes;
use crate::model::mj_wrapper::MjWrapperAttributes;
use crate::model::{
    build_indentation, Css, MjBaseComponent, MjBaseComponentAttributes, MjBaseComponentSimplified,
    SizePx, Url,
};
use std::collections::HashMap;
use std::io::Write;

pub struct MjHead {
    pub mj_attributes: Option<MjAttributes>,
    pub mj_breakpoint: Option<SizePx>,
    pub mj_font: Vec<MjFont>,
    pub mj_html_attributes: Vec<MjSelector>,
    pub mj_preview: Option<String>,
    pub mj_style: Option<MjStyle>,
    pub mj_title: Option<String>,
}

#[derive(Default)]
pub struct MjAttributes {
    pub content: Vec<MjAttribute>,
}

pub struct MjClass {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

pub enum MjAttribute {
    MjAll(HashMap<String, String>),
    MjClass(MjClass),
    MjAccordion(MjAccordionAttributes),
    MjAccordionElement(MjAccordionElementAttributes),
    MjAccordionElementTitle(MjAccordionElementTitleAttributes),
    MjAccordionElementText(MjAccordionElementTextAttributes),
    MjButton(MjButtonAttributes),
    MjCarousel(MjCarouselAttributes),
    MjCarouselImage(MjCarouselImageAttributes),
    MjColumn(MjColumnAttributes),
    MjDivider(MjDividerAttributes),
    MjGroup(MjGroupAttributes),
    MjHero(MjHeroAttributes),
    MjImage(MjImageAttributes),
    MjNavbar(MjNavbarAttributes),
    MjNavbarLink(MjNavbarLinkAttributes),
    MjSection(MjSectionAttributes),
    MjSocial(MjSocialAttributes),
    MjSocialElement(MjSocialElementAttributes),
    MjSpacer(MjSpacerAttributes),
    MjTable(MjTableAttributes),
    MjText(MjTextAttributes),
    MjWrapper(MjWrapperAttributes),
}

pub struct MjFont {
    name: String,
    href: Url,
}

pub struct MjSelector {
    pub path: String,
    pub mj_html_attributes: Vec<MjHtmlAttribute>,
}

pub struct MjHtmlAttribute {
    pub name: String,
    pub value: String,
}

pub struct MjStyle {
    pub inline: bool,
    pub content: Css,
}

impl MjBaseComponentSimplified for MjHead {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        let indentation = build_indentation(depth);
        if let Some(mj_attributes) = &self.mj_attributes {
            mj_attributes.render(writer, depth)?;
        }
        if let Some(breakpoint) = &self.mj_breakpoint {
            writeln!(
                writer,
                "{}<mj-breakpoint width=\"{}\" />",
                indentation, breakpoint
            )?;
        }
        if !self.mj_font.is_empty() {
            for font in &self.mj_font {
                font.render(writer, depth)?;
            }
        }
        if !self.mj_html_attributes.is_empty() {
            writeln!(writer, "{}<mj-html-attributes>", indentation)?;
            for attribute in &self.mj_html_attributes {
                attribute.render(writer, depth + 1)?;
            }
            writeln!(writer, "{}</mj-html-attributes>", indentation)?;
        }
        if let Some(preview) = &self.mj_preview {
            writeln!(
                writer,
                "{}<mj-preview>{}</mj-preview>",
                indentation, preview
            )?;
        }
        if let Some(mj_style) = &self.mj_style {
            mj_style.render(writer, depth)?;
        }
        if let Some(title) = &self.mj_title {
            writeln!(writer, "{}<mj-title>{}</mj-title>", indentation, title)?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-head"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        None
    }
}

impl MjBaseComponentSimplified for MjAttributes {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for attribute in &self.content {
            attribute.render(writer, depth)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-attributes"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        None
    }
}

impl MjBaseComponent for MjAttribute {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        let (name, attr): (&str, &dyn MjBaseComponentAttributes) = match self {
            MjAttribute::MjAll(attributes) => return render_mj_all(writer, depth, attributes),
            MjAttribute::MjClass(class) => return render_mj_class(writer, depth, class),
            MjAttribute::MjAccordion(attr) => ("mj-accordion", attr),
            MjAttribute::MjAccordionElement(attr) => ("mj-accordion-element", attr),
            MjAttribute::MjAccordionElementTitle(attr) => ("mj-accordion-title", attr),
            MjAttribute::MjAccordionElementText(attr) => ("mj-accordion-text", attr),
            MjAttribute::MjButton(attr) => ("mj-button", attr),
            MjAttribute::MjCarousel(attr) => ("mj-carousel", attr),
            MjAttribute::MjCarouselImage(attr) => ("mj-carousel-image", attr),
            MjAttribute::MjColumn(attr) => ("mj-column", attr),
            MjAttribute::MjDivider(attr) => ("mj-divider", attr),
            MjAttribute::MjGroup(attr) => ("mj-group", attr),
            MjAttribute::MjHero(attr) => ("mj-hero", attr),
            MjAttribute::MjImage(attr) => ("mj-image", attr),
            MjAttribute::MjNavbar(attr) => ("mj-navbar", attr),
            MjAttribute::MjNavbarLink(attr) => ("mj-navbar-link", attr),
            MjAttribute::MjSection(attr) => ("mj-section", attr),
            MjAttribute::MjSocial(attr) => ("mj-social", attr),
            MjAttribute::MjSocialElement(attr) => ("mj-social-element", attr),
            MjAttribute::MjSpacer(attr) => ("mj-spacer", attr),
            MjAttribute::MjTable(attr) => ("mj-table", attr),
            MjAttribute::MjText(attr) => ("mj-text", attr),
            MjAttribute::MjWrapper(attr) => ("mj-wrapper", attr),
        };

        let indentation = build_indentation(depth);
        write!(writer, "{}<{}", indentation, name)?;
        attr.render(writer)?;
        write!(writer, "/>")
    }
}

fn render_mj_all(
    writer: &mut dyn Write,
    depth: usize,
    attributes: &HashMap<String, String>,
) -> std::io::Result<()> {
    let indentation = build_indentation(depth);
    write!(writer, "{}<mj-all", indentation)?;
    for (key, value) in attributes {
        write!(writer, " {}=\"{}\"", key, value)?;
    }
    write!(writer, "/>")
}
fn render_mj_class(writer: &mut dyn Write, depth: usize, class: &MjClass) -> std::io::Result<()> {
    let indentation = build_indentation(depth);
    write!(writer, "{}<mj-all name=\"{}\"", indentation, class.name)?;
    for (key, value) in &class.attributes {
        write!(writer, " {}=\"{}\"", key, value)?;
    }
    write!(writer, "/>")
}

impl MjBaseComponent for MjSelector {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        if self.mj_html_attributes.is_empty() {
            return Ok(());
        }

        let indentation = build_indentation(depth);

        writeln!(
            writer,
            "{}<mj-selector path=\"{}\">",
            indentation, self.path
        )?;
        for attribute in &self.mj_html_attributes {
            writeln!(
                writer,
                "{}\t<mj-html-attribute name=\"{}\">{}</mj-html-attribute>",
                indentation, attribute.name, attribute.value
            )?;
        }
        writeln!(writer, "{}</mj-selector>", indentation)
    }
}

impl MjBaseComponent for MjStyle {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        let indentation = build_indentation(depth);
        let inline = if self.inline { "inline=\"inline\"" } else { "" };

        writeln!(writer, "{}<mj-style {}>", indentation, inline)?;
        writeln!(writer, "{}", self.content)?;
        writeln!(writer, "{}</mj-style>", indentation)
    }
}

impl MjBaseComponent for MjFont {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        let indentation = build_indentation(depth);
        writeln!(
            writer,
            "{}<mj-font name=\"{}\"  href=\"{}\" />",
            indentation, self.name, self.href
        )
    }
}
