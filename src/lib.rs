extern crate serde;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

extern crate csv;
use csv::{ReaderBuilder, Writer};

extern crate snafu;
use snafu::{ensure, ResultExt, Snafu};

pub mod command;
pub mod create;
pub mod latex;
pub mod mark;
pub mod setup;
pub mod utilities;

pub use command::{parse_arguments, Config};
pub use create::create_papers;
pub use mark::mark;
pub use setup::create_profile;
pub use utilities::backup;

// Error types

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
//#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    OpenFileErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not save file {}: {}", filename.display(), source))]
    SaveFileErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    CsvWriterErr {
        filename: PathBuf,
        source: csv::Error,
    },
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    CsvReaderErr {
        filename: PathBuf,
        source: csv::Error,
    },
    #[snafu(display("Could not deserialize csv file {}: {}", filename.display(), source))]
    CsvDeserializeErr {
        filename: PathBuf,
        source: csv::Error,
    },
    #[snafu(display("Could not serialize csv file {}: {}", filename.display(), source))]
    CsvSerializeErr {
        filename: PathBuf,
        source: csv::Error,
    },

    #[snafu(display("Number of answers of AM: {} in file {} is wrong", am, filename.display()))]
    WrongNumberOfAnswers { filename: PathBuf, am: usize },
    #[snafu(display("Could not save file {}: {}", filename.display(), source))]
    CopyFileErr {
        filename: PathBuf,
        source: std::io::Error,
    },
}

// Structs for building an exam

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub ans: String,
    pub correct: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub group: String,
    pub statement: String,
    pub answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exam {
    pub name: String,
    pub footer: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamGroupProfile {
    pub group: String,
    pub num: usize,
    pub save_space: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamProfile {
    pub total: usize,
    pub profile: Vec<ExamGroupProfile>,
}

type MarkProfile = HashMap<String, Marks>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Paper {
    serial: usize,
    questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Marks {
    correct_mark: f64,
    wrong_mark: f64,
}

const EXAM_PROFILE_JSON: &str = "examProfile.json";

const MARK_PROFILE_JSON: &str = "markProfile.json";

const ALL_QUESTIONS_TEX: &str = "ALL_QUESTIONS.tex";

const TEST_PAPERS_TEX: &str = "TEST_PAPERS.tex";

pub const TEST_PAPERS_JSON: &str = "_testPapers.json";

const CORRECT_ANSWERS_CSV: &str = "CORRECT_ANSWERS.csv";

const CORRECT_ANSWERS_JSON: &str = "_correctAnswers.json";

const RESULTS_CSV: &str = "RESULTS.csv";

const LATEX_PREAMBLE: &str = "_latexPreamble.tex";

const BACKUP_DIR: &str = "exam_backup";
