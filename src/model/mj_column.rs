use crate::model::{
    render_css_class, render_generic_attribute, render_padding, BorderProperties, Color,
    ColumnCompatible, CssClass, MjBaseComponentAttributes, MjBaseComponentSimplified,
    MjBodyColumnComponent, MjBodyComponent, Padding, PxOrPercent, VerticalAlignment,
};
use std::io::Write;

pub struct MjColumn {
    // Cannot contain MjColumn or MjSection
    pub content: Vec<Box<dyn MjBodyComponent>>,
    pub attributes: MjColumnAttributes,
}

#[derive(Default)]
pub struct MjColumnAttributes {
    pub background_color: Option<Color>,
    pub inner_background_color: Option<Color>,
    pub border: Option<BorderProperties>,
    pub border_bottom: Option<BorderProperties>,
    pub border_left: Option<BorderProperties>,
    pub border_right: Option<BorderProperties>,
    pub border_top: Option<BorderProperties>,
    pub border_radius: Option<PxOrPercent>,

    pub width: Option<PxOrPercent>, // only percent allowed in group
    pub vertical_align: Option<VerticalAlignment>,
    pub padding: Option<MjColumnPaddingAttributes>,
    pub css_class: Vec<CssClass>,
}

pub struct MjColumnPaddingAttributes {
    pub inner_border: Option<BorderProperties>,
    pub inner_border_bottom: Option<BorderProperties>,
    pub inner_border_left: Option<BorderProperties>,
    pub inner_border_right: Option<BorderProperties>,
    pub inner_border_top: Option<BorderProperties>,
    pub inner_border_radius: Option<PxOrPercent>,

    pub padding: Padding,
}

impl MjBodyColumnComponent for MjColumn {
    fn to_column(&self) -> ColumnCompatible {
        ColumnCompatible::Column(self)
    }
}
impl MjBaseComponentSimplified for MjColumn {
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
        "mj-column"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        Some(&self.attributes)
    }
}

impl MjBaseComponentAttributes for MjColumnAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(
            writer,
            "inner-background-color",
            &self.inner_background_color,
        )?;
        render_generic_attribute(writer, "border", &self.border)?;
        render_generic_attribute(writer, "border-bottom", &self.border_bottom)?;
        render_generic_attribute(writer, "border-left", &self.border_left)?;
        render_generic_attribute(writer, "border-right", &self.border_right)?;
        render_generic_attribute(writer, "border-top", &self.border_top)?;
        render_generic_attribute(writer, "border-radius", &self.border_radius)?;
        render_generic_attribute(writer, "width", &self.width)?;
        render_generic_attribute(writer, "vertical-align", &self.vertical_align)?;
        if let Some(padding) = &self.padding {
            Box::new(padding).render(writer)?;
        }
        render_css_class(writer, &self.css_class)?;
        Ok(())
    }
}

impl MjBaseComponentAttributes for MjColumnPaddingAttributes {
    fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        render_generic_attribute(writer, "inner-border", &self.inner_border)?;
        render_generic_attribute(writer, "inner-border-bottom", &self.inner_border_bottom)?;
        render_generic_attribute(writer, "inner-border-left", &self.inner_border_left)?;
        render_generic_attribute(writer, "inner-border-right", &self.inner_border_right)?;
        render_generic_attribute(writer, "inner-border-top", &self.inner_border_top)?;
        render_generic_attribute(writer, "inner-border-radius", &self.inner_border_radius)?;
        render_padding(writer, "padding", &self.padding)?;

        Ok(())
    }
}
