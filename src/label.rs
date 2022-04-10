use crate::song;
use docx_rs::{Docx, IndentLevel, NumberingId, Paragraph};
use std::fmt::{Display, Formatter};

pub enum Label {
    //选择题
    Choice,
    //填空题
    Fill,
    //判断题
    Judge,
    //简答题
    ShortAnswer,
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::Choice => write!(f, "选择题"),
            Label::Fill => write!(f, "填空题"),
            Label::Judge => write!(f, "判断题"),
            Label::ShortAnswer => write!(f, "简答题"),
        }
    }
}

impl Label {
    pub fn paragraph(&self) -> Paragraph {
        let label = song().size(28).add_text(self.to_string());
        // .add_break(BreakType::TextWrapping);

        Paragraph::new()
            .add_run(label)
            .size(28)
            .numbering(NumberingId::new(2), IndentLevel::new(0))
    }
}

pub trait AddLabel {
    fn add_label(self, label: &Label) -> Self;
}

impl AddLabel for Docx {
    fn add_label(self, label: &Label) -> Self {
        self.add_paragraph(label.paragraph())
    }
}
