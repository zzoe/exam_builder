use crate::error::Error;
use docx_rs::{
    AbstractNumbering, AlignmentType, BreakType, Docx, IndentLevel, Level, LevelJc, LevelText,
    NumberFormat, Numbering, NumberingId, Paragraph, Run, RunFonts, SpecialIndentType, Start,
};

#[derive(Clone)]
pub struct Section {
    pub title: String,
    pub questions: Vec<Question>,
    pub numbering_id: usize,
}

#[derive(Clone, Default)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
}

impl Question {
    fn paragraph(&self, lines: usize, numbering_id: usize) -> Paragraph {
        let len = self.options.len();
        let mut question = song().size(24).add_text(&self.question);
        if len > 0 {
            question = question.add_break(BreakType::TextWrapping)
        }

        let mut p = Paragraph::new().add_run(question);

        let mut options = self
            .options
            .iter()
            .enumerate()
            .fold(song().size(21), |run, (i, opt)| {
                let r = run.add_text(opt);
                if i + 1 < len {
                    r.add_break(BreakType::TextWrapping)
                } else {
                    r
                }
            });

        options = (0..lines).fold(options, |r, _| r.add_break(BreakType::TextWrapping));

        p = p.add_run(options);

        p.size(24)
            .numbering(NumberingId::new(numbering_id), IndentLevel::new(1))
    }
}

fn song() -> Run {
    Run::new().fonts(RunFonts::new().east_asia("宋体"))
}

pub trait DocxExt {
    fn add_question(self, section: &Section) -> Self;
    fn add_answer(self, section: &Section) -> Self;
    fn add_section(self, section: &Section, is_question: bool) -> Self;
    fn add_exam_no(self, exam_no: u64) -> Self;
    fn add_custom_numbering(self, numbering_id: usize) -> Self;
    fn save(self, path: &str) -> Result<(), Error>;
}

impl DocxExt for Docx {
    fn add_question(self, section: &Section) -> Self {
        self.add_section(section, true)
    }

    fn add_answer(self, section: &Section) -> Self {
        self.add_section(section, false)
    }

    fn add_section(mut self, section: &Section, is_question: bool) -> Self {
        let title = song().size(28).add_text(&section.title);

        let title_paragraph = Paragraph::new()
            .add_run(title)
            .size(28)
            .numbering(NumberingId::new(section.numbering_id), IndentLevel::new(0));

        self = self.add_paragraph(title_paragraph);
        let len = section.questions.len();

        let mut lines = 0;
        if "简答题".eq(&section.title) && is_question {
            lines = 12;
        }

        section
            .questions
            .iter()
            .enumerate()
            .fold(self, |docx, (i, question)| {
                if i + 1 == len {
                    lines = 1;
                }
                docx.add_paragraph(question.paragraph(lines, section.numbering_id))
            })
    }

    fn add_exam_no(self, exam_no: u64) -> Self {
        self.add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("No: ").add_text(exam_no.to_string()))
                .align(AlignmentType::Right),
        )
    }

    fn add_custom_numbering(self, numbering_id: usize) -> Self {
        self.add_abstract_numbering(
            AbstractNumbering::new(numbering_id)
                .add_level(
                    Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("chineseCounting"),
                        LevelText::new("%1、"),
                        LevelJc::new("left"),
                    )
                    .indent(None, None, None, None),
                )
                .add_level(
                    Level::new(
                        1,
                        Start::new(1),
                        NumberFormat::new("decimal"),
                        LevelText::new("%2."),
                        LevelJc::new("left"),
                    )
                    .indent(
                        None,
                        Some(SpecialIndentType::Hanging(300)),
                        None,
                        Some(150),
                    ),
                ),
        )
        .add_numbering(Numbering::new(numbering_id, numbering_id))
    }

    fn save(self, path: &str) -> Result<(), Error> {
        let path = std::path::Path::new(path);
        let file = std::fs::File::create(&path).unwrap();
        self.add_custom_numbering(2)
            .build()
            .pack(file)
            .map_err(|e| Error::SaveFail(e.to_string()))
    }
}
