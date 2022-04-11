use docx_rs::{
    AbstractNumbering, Docx, Level, LevelJc, LevelText, NumberFormat, Numbering, SpecialIndentType,
    Start,
};

use exam_builder::title::{AddTitle, Title};
use exam_builder::question::{AddQuestion, Question};

#[test]
fn question() {
    let l1 = Title::new("单选题");
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

    let l2 = Title::new("多选题");
    let q2_1 = Question {
        solution: "A".to_string(),
        text: "请问以下描述正确的是?1".to_string(),
        answers: vec![
            "A. 42".to_string(),
            "B. The answer to life".to_string(),
            "C. the universe".to_string(),
            "D. everything".to_string(),
        ],
    };
    let q2_2 = Question {
        solution: "B".to_string(),
        text: "请问以下描述正确的是?2".to_string(),
        answers: vec![
            "A. 42".to_string(),
            "B. The answer to life".to_string(),
            "C. the universe".to_string(),
            "D. everything".to_string(),
        ],
    };

    let l3 = Title::new("判断题");
    let q3_1 = Question {
        solution: "对".to_string(),
        text: "判断题3".to_string(),
        answers: Vec::new(),
    };

    let l4 = Title::new("填空题");
    let q4_1 = Question {
        solution: "填空答案".to_string(),
        text: "填空题4".to_string(),
        answers: Vec::new(),
    };

    let l5 = Title::new("简答题");
    let q5_1 = Question {
        solution: "问答答案".to_string(),
        text: "问答题5".to_string(),
        answers: Vec::new(),
    };

    let path = std::path::Path::new("./question_test.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_title(&l1)
        .add_questions(vec![q1_1, q1_2].as_slice())
        .add_title(&l2)
        .add_questions(vec![q2_1, q2_2].as_slice())
        .add_title(&l3)
        .add_questions(vec![q3_1].as_slice())
        .add_title(&l4)
        .add_questions(vec![q4_1].as_slice())
        .add_title(&l5)
        .add_questions(vec![q5_1].as_slice())
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
