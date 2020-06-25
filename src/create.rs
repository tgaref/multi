use super::*;
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom; 
use csv::Writer;

pub fn create_papers(questions_file: &str) -> io::Result<()> {
    // Read the exam and the exam profile into native structs
    let s = fs::read_to_string(questions_file)?;
    let exam: Exam = serde_json::from_str(&s).expect(&format!("File {} is not in valid json format", questions_file));
    let s = fs::read_to_string(EXAM_PROFILE_JSON)?;
    let exam_profile: ExamProfile = serde_json::from_str(&s).expect(&format!("File {} is not in valid json format", EXAM_PROFILE_JSON));

    // Build the test papers
    let test_papers = build_test_papers(&exam, &exam_profile);
    fs::write(TEST_PAPERS_JSON, serde_json::to_string_pretty(&test_papers).expect("Failed to deserialize test papers"))?;

    // Find the correct answers for the test papers
    let mut correct = HashMap::new();
    for paper in &test_papers {
	correct.insert(paper.serial, correct_answers(paper));
    }

    // Write the correct answers in a json file (for use in marking)
    fs::write(CORRECT_ANSWERS_JSON, serde_json::to_string_pretty(&correct).expect("Failed to deserialize correct answers hash map"))?;

    // Write the correct answers in a csv file (for possible posting)
    write_csv(&correct, CORRECT_ANSWERS_CSV);

    // Write latex files
    latex::write_all_questions(&exam, ALL_QUESTIONS_TEX)?;
    latex::write_test_papers(&exam, &test_papers, &exam_profile, TEST_PAPERS_TEX)?;
    
    Ok(())    
}

fn write_csv(correct: &HashMap<usize, Vec<(usize, String)>>, filename: &str) {
    let mut wrt = Writer::from_path(filename).expect(&format!("Failed to open file {}", filename));
    let mut ans: Vec<String>;
    for (serial, vec) in correct {
	ans = vec![serial.to_string()];
	for (j,(i, _)) in vec.iter().enumerate() {
	    ans.push(format!("{}: {}",j+1,i));
	}
	wrt.write_record(&ans).expect(&format!("Failed to write {} file", filename));
    }
    wrt.flush().expect(&format!("Failed to write {} file", filename));
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

fn build_paper(serial: usize,
	       groups: &HashMap<String, Vec<Question>>,
	       question_numbers: &HashMap<String, usize>)
	       -> Paper
{ 
    let mut chosen_questions = Vec::new();
    for (group, questions) in groups {
	let n = question_numbers[group];
	if n > questions.len() {
	    panic!("Group {} has fewer that (the requested) {} questions...", group, n);
	}
	chosen_questions.extend(pick(questions, n))
    }
    let rng = &mut thread_rng();
    chosen_questions.shuffle(rng);
    Paper { serial, questions: chosen_questions }
}

fn find_correct_answer(question: &Question) -> usize {
    for (answer, i) in question.answers.iter().zip(1..) {
	if answer.correct {
	    return i
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
