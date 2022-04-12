use std::iter::repeat;
use std::time::SystemTime;

use calamine::{open_workbook, Reader, Xlsx};
use docx_rs::{BreakType, Docx, Paragraph, Run};

use crate::error::Error;
use crate::error::Error::SaveFail;
use crate::section::{DocxExt, Question, Section};

pub struct Builder(String);

impl Builder {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn build(&self, out_path: &str) -> Result<(), Error> {
        let exam_no = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut workbook: Xlsx<_> = open_workbook(&*self.0)?;
        let mut docx = Docx::new().add_exam_no(exam_no);
        let mut answers = Vec::new();

        for (title_count, range) in workbook.worksheets() {
            let mut split = title_count.split('|');
            let title = split.next().unwrap_or_default();
            let need = split.next().unwrap_or("0").parse::<usize>().unwrap();

            let rows = range.rows();
            let rands = rand_usize(rows.len(), need);

            let mut question_section = Section {
                title: title.to_string(),
                questions: repeat(Question::default()).take(rands.len()).collect(),
                numbering_id: 2,
            };
            let mut answer_section = question_section.clone();
            answer_section.numbering_id = 3;

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
                    let mut answer = Question::default();
                    if let Some(v) = row.get(0) {
                        question.question = v.to_string()
                    }
                    if let Some(v) = row.get(1) {
                        answer.question = v.to_string()
                    }

                    if row.len() > 2 {
                        for cell in &row[2..] {
                            question.options.push(cell.to_string());
                        }
                    }

                    if let Some(v) = question_section.questions.get_mut(idx) {
                        *v = question;
                    }
                    if let Some(v) = answer_section.questions.get_mut(idx) {
                        *v = answer;
                    }
                }
            }

            docx = docx.add_question(&question_section);
            answers.push(answer_section);
        }

        let path = std::path::Path::new(out_path);
        let file = std::fs::File::create(&path).unwrap();

        docx = docx
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
            .add_exam_no(exam_no);

        docx = answers
            .iter()
            .fold(docx, |docx, answer| docx.add_answer(answer));

        docx.add_custom_numbering(2)
            .add_custom_numbering(3)
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
