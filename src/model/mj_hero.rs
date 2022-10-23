use crate::model::{
    render_generic_attribute, render_padding_opt, Alignment, BaseComponent, Color,
    MjBaseComponentAttributes, MjBaseComponentSimplified, MjBodyComponent, MjBodySectionComponent,
    Padding, SectionCompatible, SizePx, Url, VerticalAlignment,
};
use std::io::Write;

pub struct MjHero {
    pub content: Vec<Box<dyn MjBodyComponent>>,
    pub attributes: MjHeroAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroMode {
    FluidHeight,
    FixedHeight(SizePx),
}

pub struct MjHeroAttributes {
    pub background_height: SizePx,
    pub background_width: SizePx,
    pub background_color: Color,
    pub background_url: Option<Url>,
    pub background_position: Option<(Alignment, VerticalAlignment)>,
    pub border_radius: Option<SizePx>,
    pub mode: Option<HeroMode>,
    pub padding: Option<Padding>,
    pub vertical_align: Option<VerticalAlignment>,
}

impl MjBodyComponent for MjHero {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}
impl MjBodySectionComponent for MjHero {
    fn to_section(&self) -> SectionCompatible {
        SectionCompatible::Hero(self)
    }
}
impl MjBaseComponentSimplified for MjHero {
    fn render_content(&self, writer: &mut dyn Write, depth: usize) -> std::io::Result<()> {
        for component in &self.content {
            component
                .to_base_component()
                .inner
                .render(writer, depth + 1)?
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "mj-hero"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjHeroAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        write!(writer, " background-height=\"{}\"", self.background_height)?;
        write!(writer, " background-width=\"{}\"", self.background_width)?;
        write!(writer, " background-color=\"{}\"", self.background_color)?;
        render_generic_attribute(writer, "background-url", &self.background_url)?;
        if let Some((align, vertical_align)) = self.background_position {
            write!(
                writer,
                " background-position=\"{} {}\"",
                align, vertical_align
            )?;
        }
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;

        if let Some(mode) = self.mode {
            match mode {
                HeroMode::FluidHeight => write!(writer, " mode=\"fluid-height\"")?,
                HeroMode::FixedHeight(height) => {
                    write!(writer, " mode=\"fixed-height\"")?;
                    write!(writer, " height=\"{}\"", height)?;
                }
            }
        }
        render_padding_opt(writer, "padding", &self.padding)?;
        render_generic_attribute(writer, "vertical-align", &self.vertical_align)?;
        Ok(())
    }
}
