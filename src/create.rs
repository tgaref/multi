use super::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::Command;

pub fn create_papers<P>(questions_file: P) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    // Read the exam and the exam profile into native structs
    let filename = questions_file.as_ref();
    let s = fs::read_to_string(filename).context(OpenFileErr {
        filename: filename.to_path_buf(),
    })?;
    let exam: Exam = serde_json::from_str(&s).expect(&format!(
        "File {} is not in valid json format",
        questions_file
    ));
    let s = fs::read_to_string(EXAM_PROFILE_JSON).context(OpenFileErr {
        filename: PathBuf::from(EXAM_PROFILE_JSON),
    })?;
    let exam_profile: ExamProfile = serde_json::from_str(&s).expect(&format!(
        "File {} is not in valid json format",
        EXAM_PROFILE_JSON
    ));

    // Build the test papers
    let test_papers = build_test_papers(&exam, &exam_profile);
    fs::write(
        TEST_PAPERS_JSON,
        serde_json::to_string_pretty(&test_papers).expect("Failed to deserialize test papers"),
    )
    .context(SaveFileErr {
        filename: PathBuf::from(TEST_PAPERS_JSON),
    })?;

    // Find the correct answers for the test papers
    let mut correct = HashMap::new();
    for paper in &test_papers {
        correct.insert(paper.serial, correct_answers(paper));
    }

    // Write the correct answers in a json file (for use in marking)
    fs::write(
        CORRECT_ANSWERS_JSON,
        serde_json::to_string_pretty(&correct)
            .expect("Failed to deserialize correct answers hash map"),
    )
    .context(SaveFileErr {
        filename: PathBuf::from(CORRECT_ANSWERS_JSON),
    })?;

    // Write the correct answers in a csv file (for possible posting)
    write_csv(&correct, CORRECT_ANSWERS_CSV)?;

    // Write latex files
    latex::write_all_questions(&exam, ALL_QUESTIONS_TEX)?;
    latex::write_test_papers(&exam, &test_papers, &exam_profile, TEST_PAPERS_TEX)?;

    Command::new("xelatex")
        .arg(ALL_QUESTIONS_TEX)
        .output()
        .expect(&format!("Failed to xelatex {} file", ALL_QUESTIONS_TEX));

    Command::new("xelatex")
        .arg(TEST_PAPERS_TEX)
        .output()
        .expect(&format!("Failed to xelatex {} file", TEST_PAPERS_TEX));

    Command::new("xelatex")
        .arg(TEST_PAPERS_TEX)
        .output()
        .expect(&format!("Failed to xelatex {} file", TEST_PAPERS_TEX));

    fs::remove_file(Path::new(ALL_QUESTIONS_TEX).with_extension("aux"))
        .expect("Failed to delete aux file");
    fs::remove_file(Path::new(ALL_QUESTIONS_TEX).with_extension("log"))
        .expect("Failed to delete log file");
    fs::remove_file(Path::new(TEST_PAPERS_TEX).with_extension("aux"))
        .expect("Failed to delete aux file");
    fs::remove_file(Path::new(TEST_PAPERS_TEX).with_extension("log"))
        .expect("Failed to delete aux file");

    Ok(())
}

fn write_csv<P>(correct: &HashMap<usize, Vec<(usize, String)>>, filename: P) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    let filename = &filename.as_ref();
    let mut wrt = Writer::from_path(filename).context(CsvWriterErr {
        filename: filename.to_path_buf(),
    })?;
    let mut ans: Vec<String>;
    for (serial, vec) in correct {
        ans = vec![serial.to_string()];
        for (j, (i, _)) in vec.iter().enumerate() {
            ans.push(format!("{}: {}", j + 1, i));
        }
        wrt.write_record(&ans).context(CsvWriterErr {
            filename: filename.to_path_buf(),
        })?;
    }
    wrt.flush().context(SaveFileErr {
        filename: filename.to_path_buf(),
    })?;

    Ok(())
}

fn build_test_papers(exam: &Exam, exam_profile: &ExamProfile) -> Vec<Paper> {
    let mut groups: HashMap<String, Vec<Question>> = HashMap::new();
    for question in &exam.questions {
        if let Some(v) = groups.get_mut(&question.group) {
            v.push(question.clone());
        } else {
            groups.insert(question.group.clone(), vec![question.clone()]);
        }
    }
    let mut question_numbers = HashMap::new();
    for group_profile in &exam_profile.profile {
        question_numbers.insert(group_profile.group.clone(), group_profile.num);
    }

    let mut test_papers = Vec::new();
    for i in 0..exam_profile.total {
        test_papers.push(build_paper(i, &groups, &question_numbers));
    }
    test_papers
}

fn pick(questions: &Vec<Question>, n: usize) -> Vec<Question> {
    let mut rng = &mut thread_rng();
    let mut chosen: Vec<Question> = questions.choose_multiple(&mut rng, n).cloned().collect();
    for question in &mut chosen {
        question.answers.shuffle(rng);
    }
    chosen
}

fn build_paper(
    serial: usize,
    groups: &HashMap<String, Vec<Question>>,
    question_numbers: &HashMap<String, usize>,
) -> Paper {
    let mut chosen_questions = Vec::new();
    for (group, questions) in groups {
        let n = question_numbers[group];
        if n > questions.len() {
            panic!(
                "Group {} has fewer that (the requested) {} questions...",
                group, n
            );
        }
        chosen_questions.extend(pick(questions, n))
    }
    let rng = &mut thread_rng();
    chosen_questions.shuffle(rng);
    Paper {
        serial,
        questions: chosen_questions,
    }
}

fn find_correct_answer(question: &Question) -> usize {
    for (answer, i) in question.answers.iter().zip(1..) {
        if answer.correct {
            return i;
        }
    }
    panic!("Question {:?} has no correct answer", question)
}

fn correct_answers(paper: &Paper) -> Vec<(usize, String)> {
    let mut correct_vec = Vec::new();
    for question in &paper.questions {
        correct_vec.push((find_correct_answer(question), question.group.clone()));
    }
    correct_vec
}
