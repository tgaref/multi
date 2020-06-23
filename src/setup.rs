use crate::*;
use std::collections::HashMap;

fn setup(exam: Exam) -> (ExamProfile, MarkProfile) {
    let mut groups = HashMap::new();
    for question in exam.questions {
	if let Some(n) = groups.get_mut(&question.group) {
	    *n += 1;
	} else {
	    groups.insert(question.group.clone(), 1);
	}
    }
    let mut eprofile = Vec::new();
    let mut mprofile = Vec::new();
    for (group, num) in groups {
	eprofile.push(ExamGroupProfile { group: group.clone(), num, save_space: false });
	mprofile.push(MarkGroupProfile { group: group.clone(), correct_mark: 1.0, wrong_mark: -0.25 });
    }
    let exam_profile = ExamProfile {
	total: 10,
	seed: 123456789,
	profile: eprofile
    };
    let mark_profile = MarkProfile {
	profile: mprofile
    };
    (exam_profile, mark_profile)
}

pub fn create_profile(questions_file: &str) -> io::Result<()> {
    let s = fs::read_to_string(questions_file)?;
    let exam: Exam = serde_json::from_str(&s).expect("The questions file is not in valid json format");
    let (exam_profile, mark_profile) = setup(exam);
    fs::write(EXAM_PROFILE_JSON, serde_json::to_string(&exam_profile).expect("Failed to deserialize exam profile"))?;
    fs::write(MARK_PROFILE_JSON, serde_json::to_string(&mark_profile).expect("Failed to deserialize mark profile"))
}
