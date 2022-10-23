pub mod mj_accordion;
pub mod mj_body;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_group;
pub mod mj_head;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_navbar;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_spacer;
pub mod mj_table;
pub mod mj_text;
pub mod mj_wrapper;
pub mod mjml;
pub mod utils;

use crate::model::mj_column::MjColumn;
use crate::model::mj_group::MjGroup;
use crate::model::mj_hero::MjHero;
use crate::model::mj_section::MjSection;
use crate::model::mj_wrapper::MjWrapper;
use std::io::Write;
pub use utils::*;

trait MjBaseComponent {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()>;
}

trait MjBaseComponentSimplified {
    fn render_content(&self, writer: &mut dyn std::io::Write, depth: usize) -> std::io::Result<()>;
    fn name(&self) -> &'static str;
    fn has_content(&self) -> bool;
    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes>;

    fn render_all(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        let indentation = build_indentation(depth);
        let name = self.name();
        write!(writer, "{}<{}", indentation, name)?;
        if let Some(attributes) = self.attributes() {
            attributes.render(writer)?;
        }
        if self.has_content() {
            writeln!(writer, ">")?;
            self.render_content(writer, depth + 1)?;
            writeln!(writer, "{}</{}>", indentation, name)?;
        } else {
            writeln!(writer, "/>")?;
        }
        Ok(())
    }
}

impl<T: MjBaseComponentSimplified> MjBaseComponent for T {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        self.render_all(writer, depth)
    }
}

trait MjBaseComponentAttributes {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()>;
}

pub trait MjBodyComponent {
    fn to_base_component(&self) -> BaseComponent;
}

pub struct BaseComponent<'a> {
    inner: &'a dyn MjBaseComponent,
}

pub enum SectionCompatible<'a> {
    Section(&'a MjSection),
    Wrapper(&'a MjWrapper),
    Hero(&'a MjHero),
}

impl<'a> MjBaseComponent for SectionCompatible<'a> {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        match *self {
            SectionCompatible::Section(s) => Box::new(s).render(writer, depth),
            SectionCompatible::Wrapper(w) => Box::new(w).render(writer, depth),
            SectionCompatible::Hero(h) => Box::new(h).render(writer, depth),
        }
    }
}

pub trait MjBodySectionComponent {
    fn to_section(&self) -> SectionCompatible;
}

pub enum ColumnCompatible<'a> {
    Column(&'a MjColumn),
    Group(&'a MjGroup),
}

impl<'a> MjBaseComponent for ColumnCompatible<'a> {
    fn render(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        match self {
            ColumnCompatible::Column(c) => c.render(writer, depth),
            ColumnCompatible::Group(g) => g.render(writer, depth),
        }
    }
}

pub trait MjBodyColumnComponent {
    fn to_column(&self) -> ColumnCompatible;
}
