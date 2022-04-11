use docx_rs::{Docx, IndentLevel, NumberingId, Paragraph};

use crate::song;

pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> Self {
        Title(title.to_string())
    }

    pub fn paragraph(&self) -> Paragraph {
        let label = song().size(28).add_text(self.0.as_str());

        Paragraph::new()
            .add_run(label)
            .size(28)
            .numbering(NumberingId::new(2), IndentLevel::new(0))
    }
}

pub trait AddTitle {
    fn add_title(self, title: &Title) -> Self;
}

impl AddTitle for Docx {
    fn add_title(self, title: &Title) -> Self {
        self.add_paragraph(title.paragraph())
    }
}
