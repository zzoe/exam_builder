use calamine::{open_workbook, Reader, Xlsx};
use docx_rs::{
    AbstractNumbering, Docx, Level, LevelJc, LevelText, NumberFormat, Numbering, SpecialIndentType,
    Start,
};
use std::iter::repeat;

use crate::error::Error;
use crate::error::Error::SaveFail;
use crate::section::{AddSection, Question, Section};

pub struct Builder(String);

impl Builder {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn build(&self, out_path: &str) -> Result<(), Error> {
        let mut workbook: Xlsx<_> = open_workbook(&*self.0)?;
        let mut docx = Docx::new();

        for (title_count, range) in workbook.worksheets() {
            let mut split = title_count.split('|');
            let title = split.next().unwrap_or_default();
            let need = split.next().unwrap_or("0").parse::<usize>().unwrap();

            let rows = range.rows();
            let rands = rand_usize(rows.len(), need);

            let mut section = Section {
                title: title.to_string(),
                questions: repeat(Question::default()).take(need).collect(),
            };

            for (i, row) in rows.enumerate() {
                let mut idx = need;
                rands.iter().enumerate().any(|(index, v)| {
                    if *v == i {
                        idx = index;
                        true
                    } else {
                        false
                    }
                });

                if idx != need {
                    let mut question = Question::default();
                    if let Some(v) = row.get(0) {
                        question.question = v.to_string()
                    }
                    if let Some(v) = row.get(1) {
                        question.answer = v.to_string()
                    }

                    if row.len() > 2 {
                        for cell in &row[2..] {
                            question.options.push(cell.to_string());
                        }
                    }
                    section.questions.insert(idx, question);
                }
            }

            docx = docx.add_section(&section);
        }

        let path = std::path::Path::new(out_path);
        let file = std::fs::File::create(&path).unwrap();

        docx.add_abstract_numbering(
            AbstractNumbering::new(2)
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
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)
        .map_err(|e| SaveFail(e.to_string()))?;

        Ok(())
    }
}

fn rand_usize(total: usize, need: usize) -> Vec<usize> {
    let mut pool = (0..total).collect::<Vec<_>>();

    if total <= need {
        return pool;
    }

    let rng = fastrand::Rng::new();
    let mut res = Vec::new();
    for _ in 0..need {
        let index = rng.usize(..pool.len());
        res.push(pool.remove(index));
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn rand_usize() {
        let total = 10;
        let need = 5;
        let rands = super::rand_usize(total, need);
        assert_eq!(rands.len(), need);
        assert!(rands.iter().all(|v| *v < total));
        println!("{:?}", rands);
    }
}
