use docx_rs::{BreakType, Docx, IndentLevel, NumberingId, Paragraph, Run, RunFonts};

pub struct Section {
    pub title: String,
    pub questions: Vec<Question>,
}

pub trait AddSection {
    fn add_section(self, section: &Section) -> Self;
}

impl AddSection for Docx {
    fn add_section(mut self, section: &Section) -> Self {
        let title = song().size(28).add_text(&section.title);

        let title_paragraph = Paragraph::new()
            .add_run(title)
            .size(28)
            .numbering(NumberingId::new(2), IndentLevel::new(0));

        self = self.add_paragraph(title_paragraph);
        let len = section.questions.len();
        section
            .questions
            .iter()
            .enumerate()
            .fold(self, |docx, (i, question)| {
                let last = i + 1 == len;
                docx.add_paragraph(question.paragraph(last))
            })
    }
}

#[derive(Clone, Default)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub options: Vec<String>,
}

impl Question {
    fn paragraph(&self, last: bool) -> Paragraph {
        let question = song()
            .size(24)
            .add_text(&self.question)
            .add_break(BreakType::TextWrapping);

        let mut p = Paragraph::new().add_run(question);

        let len = self.options.len();
        let answers = self
            .options
            .iter()
            .enumerate()
            .fold(song().size(21), |run, (i, opt)| {
                let r = run.add_text(opt);
                if i + 1 < len || last {
                    r.add_break(BreakType::TextWrapping)
                } else {
                    r
                }
            });

        p = p.add_run(answers);

        p.size(24)
            .numbering(NumberingId::new(2), IndentLevel::new(1))
    }
}

fn song() -> Run {
    Run::new().fonts(RunFonts::new().east_asia("宋体"))
}
