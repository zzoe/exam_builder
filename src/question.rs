use docx_rs::{BreakType, Docx, IndentLevel, NumberingId, Paragraph};

use crate::song;

pub struct Question {
    pub solution: String,
    pub text: String,
    pub answers: Vec<String>,
}

impl Question {
    pub fn paragraph(&self, last: bool) -> Paragraph {
        let question = song()
            .size(24)
            .add_text(self.text.as_str())
            .add_break(BreakType::TextWrapping);

        let mut p = Paragraph::new().add_run(question);

        let len = self.answers.len();
        let answers = self
            .answers
            .iter()
            .enumerate()
            .fold(song().size(21), |run, (i, answer)| {
                let r = run.add_text(answer);
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

pub trait AddQuestion {
    fn add_questions(self, question: &[Question]) -> Self;
}

impl AddQuestion for Docx {
    fn add_questions(self, questions: &[Question]) -> Self {
        let len = questions.len();
        questions
            .iter()
            .enumerate()
            .fold(self, |docx, (i, question)| {
                let last = i + 1 == len;
                docx.add_paragraph(question.paragraph(last))
            })
    }
}
