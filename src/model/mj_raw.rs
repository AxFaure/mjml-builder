use crate::model::{
    BaseComponent, Html, MjBaseComponentAttributes, MjBaseComponentSimplified, MjBodyComponent,
};
use std::io::Write;

pub struct MjRaw {
    pub content: Html,
}

impl MjBodyComponent for MjRaw {
    fn to_base_component(&self) -> BaseComponent {
        BaseComponent { inner: self }
    }
}

impl MjBaseComponentSimplified for MjRaw {
    fn render_content(&self, writer: &mut dyn Write, _depth: usize) -> std::io::Result<()> {
        writeln!(writer, "{}", self.content)
    }

    fn name(&self) -> &'static str {
        "mj-raw"
    }

    fn has_content(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&dyn MjBaseComponentAttributes> {
        None
    }
}
