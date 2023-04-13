use super::*;

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    serial: usize,
    am: usize,
    answer: String,
}

pub fn mark<P>(given_answers_csv: P) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    let filename = given_answers_csv.as_ref();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(filename)
        .context(CsvReaderErr {
            filename: filename.to_path_buf(),
        })?;

    let s = fs::read_to_string(CORRECT_ANSWERS_JSON).context(OpenFileErr {
        filename: PathBuf::from(CORRECT_ANSWERS_JSON),
    })?;

    // maps serial to (correct answer, group name)
    let correct_answers_map: HashMap<usize, Vec<(usize, String)>> = serde_json::from_str(&s)
        .expect(&format!(
            "File {} is not in valid json format",
            CORRECT_ANSWERS_JSON
        ));

    let s = fs::read_to_string(MARK_PROFILE_JSON).context(OpenFileErr {
        filename: PathBuf::from(MARK_PROFILE_JSON),
    })?;
    let marks_map: MarkProfile = serde_json::from_str(&s).expect(&format!(
        "File {} is not in valid json format",
        MARK_PROFILE_JSON
    ));

    let mut wrt = Writer::from_path(RESULTS_CSV).context(CsvWriterErr {
        filename: PathBuf::from(RESULTS_CSV),
    })?;

    let s = fs::read_to_string(EXAM_PROFILE_JSON).context(OpenFileErr {
        filename: PathBuf::from(EXAM_PROFILE_JSON),
    })?;
    let exam_profile: ExamProfile = serde_json::from_str(&s).expect(&format!(
        "File {} is not in valid json format",
        EXAM_PROFILE_JSON
    ));

    let mut total = 0;
    for grp in exam_profile.profile {
        total += grp.num;
    }

    for record in rdr.records() {
        let mut record = record.unwrap();
        record.trim();
        let row: Row = record.deserialize(None).context(CsvDeserializeErr {
            filename: filename.to_path_buf(),
        })?;
        let correct: &Vec<(usize, String)> = &correct_answers_map[&row.serial];

        let mut mark: f64 = 0.0;

        ensure!(
            row.answer.len() == total,
            WrongNumberOfAnswers {
                filename: filename.to_path_buf(),
                am: row.am
            }
        );

        let mut correct_string = String::new();
        for (a, (b, grp)) in row.answer.chars().zip(correct) {
            if let Some(b) = std::char::from_digit(*b as u32, 10) {
                correct_string.push(b);
                if a == '-' || a == 'x' {
                    continue;
                };
                if a == b {
                    mark += marks_map[grp].correct_mark;
                } else {
                    mark += marks_map[grp].wrong_mark;
                }
            }
        }
        wrt.write_record(&[
            row.am.to_string(),
            row.serial.to_string(),
            correct_string,
            mark.to_string(),
        ])
        .context(CsvWriterErr {
            filename: PathBuf::from(RESULTS_CSV),
        })?;
    }
    wrt.flush().context(SaveFileErr {
        filename: PathBuf::from(RESULTS_CSV),
    })?;

    Ok(())
}
