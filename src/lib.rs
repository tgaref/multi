use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use serde_json;

pub mod setup;
pub mod create;
pub mod mark;

pub use setup::create_profile;
pub use create::create_papers;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub ans: String,
    pub correct: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub group: String,
    pub statement: String,
    pub answers: Vec<Answer>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exam {
    pub name: String,
    pub footer: String,
    pub questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamGroupProfile {
    pub group: String,
    pub num: usize,
    pub save_space: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamProfile {
    pub total: usize,
    pub profile: Vec<ExamGroupProfile>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkGroupProfile {
    pub group: String,
    pub correct_mark: f64,
    pub wrong_mark: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkProfile {
    pub profile: Vec<MarkGroupProfile>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paper {
    serial: usize,
    questions: Vec<Question>
}

const EXAM_PROFILE_JSON: &str = "examProfile.json";

const MARK_PROFILE_JSON: &str = "markProfile.json";

const ALL_QUESTIONS_TEX: &str = "ALL_QUESTIONS.tex";

const TEST_PAPAERS_TEX: &str = "TEST_PAPERS.tex";

const TEST_PAPERS_F: &str = "_testPapers_";

const ASSOC_F: &str = "_assocList_";

const CORRECT_ANSWERS_CSV: &str = "CORRECT_ANSWERS.csv";

const CORRECT_ANS_F: &str = "_correctAns_";

const RESULTS_CSV: &str = "RESULTS.csv";

const LATEX_PREAMBLE: &str = "_latexPreamble_";


