use crate::model::mj_body::MjBody;
use crate::model::mj_head::MjHead;
use crate::model::{build_indentation, Html, MjBaseComponent};
use std::io::Write;

pub struct Mjml {
    pub mj_raw_file_start: Option<MjRawFileStart>,
    pub mj_body: MjBody,
    pub mj_head: Option<MjHead>,

    pub attributes: MjmlAttributes,
}

#[derive(Default)]
pub struct MjmlAttributes {
    pub owa: Option<Owa>,
    pub lang: Option<String>,
    pub dit: Option<String>,
}

pub enum Owa {
    Desktop,
}

pub struct MjRawFileStart {
    pub content: Html,
}

impl Mjml {
    pub fn render(self, writer: &mut Box<&mut dyn std::io::Write>) -> std::io::Result<()> {
        writeln!(writer, "<mjml>")?;
        if let Some(mj_raw_file_start) = self.mj_raw_file_start {
            Box::new(mj_raw_file_start).render(writer, 1)?;
        }
        if let Some(mj_head) = self.mj_head {
            Box::new(mj_head).render(writer, 1)?;
        }
        Box::new(self.mj_body).render(writer, 1)?;
        writeln!(writer, "</mjml>")?;

        writer.flush()?;

        Ok(())
    }

    pub fn render_to_string(self) -> std::io::Result<String> {
        let mut buf: Vec<u8> = Vec::new();

        self.render(&mut Box::new(&mut buf))?;

        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

impl MjBaseComponent for MjRawFileStart {
    fn render(&self, writer: &mut dyn std::io::Write, depth: usize) -> std::io::Result<()> {
        let indentation = build_indentation(depth);
        writeln!(writer, "{}<mj-raw position=\"file-start\">", indentation)?;
        writeln!(writer, "{}", self.content)?;
        writeln!(writer, "{}</mj-raw>", indentation)?;
        Ok(())
    }
}
