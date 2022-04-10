use docx_rs::{
    AbstractNumbering, Docx, Level, LevelJc, LevelText, NumberFormat, Numbering, SpecialIndentType,
    Start,
};

use exam_builder::label::{AddLabel, Label};
use exam_builder::question::{AddQuestion, Question};

#[test]
fn question() {
    let l1 = Label::Choice;
    let q1_1 = Question {
        solution: "A".to_string(),
        text: "请问以下描述正确的是?1".to_string(),
        answers: vec![
            "A. 42".to_string(),
            "B. The answer to life".to_string(),
            "C. the universe".to_string(),
            "D. everything".to_string(),
        ],
    };
    let q1_2 = Question {
        solution: "B".to_string(),
        text: "请问以下描述正确的是?2".to_string(),
        answers: vec![
            "A. 42".to_string(),
            "B. The answer to life".to_string(),
            "C. the universe".to_string(),
            "D. everything".to_string(),
        ],
    };

    let l2 = Label::Judge;
    let q2_1 = Question {
        solution: "对".to_string(),
        text: "判断题3".to_string(),
        answers: Vec::new(),
    };

    let l3 = Label::Fill;
    let q3_1 = Question {
        solution: "填空答案".to_string(),
        text: "填空题4".to_string(),
        answers: Vec::new(),
    };

    let l4 = Label::ShortAnswer;
    let q4_1 = Question {
        solution: "问答答案".to_string(),
        text: "问答题5".to_string(),
        answers: Vec::new(),
    };

    let path = std::path::Path::new("./question_test.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_label(&l1)
        .add_questions(vec![q1_1, q1_2].as_slice())
        .add_label(&l2)
        .add_questions(vec![q2_1].as_slice())
        .add_label(&l3)
        .add_questions(vec![q3_1].as_slice())
        .add_label(&l4)
        .add_questions(vec![q4_1].as_slice())
        .add_abstract_numbering(
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
        .ok();
}
