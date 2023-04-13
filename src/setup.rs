use super::*;

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
    let mut mprofile = HashMap::new();
    for (group, num) in groups {
        eprofile.push(ExamGroupProfile {
            group: group.clone(),
            num,
            save_space: false,
        });
        mprofile.insert(
            group.clone(),
            Marks {
                correct_mark: 1.0,
                wrong_mark: -0.25,
            },
        );
    }
    let exam_profile = ExamProfile {
        total: 10,
        profile: eprofile,
    };
    (exam_profile, mprofile)
}

pub fn create_profile<P>(questions_file: P) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    let filename = questions_file.as_ref();
    let s = fs::read_to_string(filename).context(OpenFileErr {
        filename: filename.to_path_buf(),
    })?;
    let exam: Exam = serde_json::from_str(&s).expect(&format!(
        "File {} is not in valid json format",
        questions_file
    ));
    let (exam_profile, mark_profile) = setup(exam);
    fs::write(
        EXAM_PROFILE_JSON,
        serde_json::to_string_pretty(&exam_profile).expect("Failed to deserialize exam profile"),
    )
    .context(SaveFileErr {
        filename: PathBuf::from(EXAM_PROFILE_JSON),
    })?;
    fs::write(
        MARK_PROFILE_JSON,
        serde_json::to_string_pretty(&mark_profile).expect("Failed to deserialize mark profile"),
    )
    .context(SaveFileErr {
        filename: PathBuf::from(EXAM_PROFILE_JSON),
    })?;
    Ok(())
}
