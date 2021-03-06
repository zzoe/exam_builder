use std::collections::HashMap;
use std::iter::repeat;
use std::time::SystemTime;

use calamine::{open_workbook, Reader, Xlsx};
use docx_rs::Docx;

use crate::error::Error;
use crate::section::{DocxExt, Question, Section};

pub struct Builder(String);

impl Builder {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn build(&self) -> Result<(), Error> {
        let exam_no = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut workbook: Xlsx<_> = open_workbook(&*self.0)?;
        let mut exam_docx = Docx::new().add_exam_no(exam_no);
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

            for (i, row) in rows.enumerate() {
                if let Some(&idx) = rands.get(&i) {
                    if let Some(question) = question_section.questions.get_mut(idx) {
                        if let Some(v) = row.get(0) {
                            question.subject = v.to_string()
                        }

                        if row.len() > 2 {
                            for cell in &row[2..] {
                                question.options.push(cell.to_string());
                            }
                        }
                    }

                    if let Some(answer) = answer_section.questions.get_mut(idx) {
                        if let Some(v) = row.get(1) {
                            answer.subject = v.to_string()
                        }
                    }
                }
            }

            exam_docx = exam_docx.add_question(&question_section);
            answers.push(answer_section);
        }

        exam_docx.save(&*format!("{}.docx", exam_no))?;

        let answer_docx = Docx::new().add_exam_no(exam_no);

        answers
            .iter()
            .fold(answer_docx, |docx, answer| docx.add_answer(answer))
            .save(&*format!("{}??????.docx", exam_no))?;

        Ok(())
    }
}

fn rand_usize(total: usize, need: usize) -> HashMap<usize, usize> {
    let mut pool = (0..total).collect::<Vec<_>>();

    let mut need = need;
    if total <= need {
        need = total;
    }

    let rng = fastrand::Rng::new();
    let mut res = HashMap::new();
    for i in 0..need {
        let index = rng.usize(..pool.len());
        res.insert(pool.remove(index), i);
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
        assert!(rands.iter().all(|v| *v.0 < total));
        println!("{:?}", rands);
    }
}
