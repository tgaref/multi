use super::*;
//use serde_json;
use std::io;
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom; 

pub fn create_papers(questions_file: &str) -> io::Result<()> {
    let s = fs::read_to_string(questions_file)?;
    let exam: Exam = serde_json::from_str(&s).expect(&format!("File {} is not in valid json format", questions_file));
    let s = fs::read_to_string(EXAM_PROFILE_JSON)?;
    let exam_profile: ExamProfile = serde_json::from_str(&s).expect(&format!("File {} is not in valid json format", EXAM_PROFILE_JSON));

    let test_papers = build_test_papers(&exam, &exam_profile);
    fs::write(TEST_PAPERS_F, serde_json::to_string_pretty(&test_papers).expect("Failed to deserialize test papers"))?;
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
    Paper { serial, questions: chosen_questions }
}
