use super::*;

pub fn backup<P>(questions: P) -> Result<()>
where
    P: AsRef<Path>
{
    if ! Path::new(BACKUP_DIR).exists() {
	fs::create_dir(BACKUP_DIR).expect("Failed to create backup directory");
    }
    fs::copy(&questions, PathBuf::from(BACKUP_DIR).join(&questions))
	.context(CopyFileErr { filename: questions.as_ref().to_path_buf() })?;
    fs::copy(EXAM_PROFILE_JSON, PathBuf::from(BACKUP_DIR).join(EXAM_PROFILE_JSON))
	.context(CopyFileErr { filename: PathBuf::from(EXAM_PROFILE_JSON) })?;
    fs::copy(MARK_PROFILE_JSON, PathBuf::from(BACKUP_DIR).join(MARK_PROFILE_JSON))
	.context(CopyFileErr { filename: PathBuf::from(MARK_PROFILE_JSON) })?;
    fs::copy(TEST_PAPERS_JSON, PathBuf::from(BACKUP_DIR).join(TEST_PAPERS_JSON))
	.context(CopyFileErr { filename: PathBuf::from(TEST_PAPERS_JSON) })?;
    fs::copy(TEST_PAPERS_TEX, PathBuf::from(BACKUP_DIR).join(TEST_PAPERS_TEX))
	.context(CopyFileErr { filename: PathBuf::from(TEST_PAPERS_TEX) })?;
    fs::copy(CORRECT_ANSWERS_CSV, PathBuf::from(BACKUP_DIR).join(CORRECT_ANSWERS_CSV))
	.context(CopyFileErr { filename: PathBuf::from(CORRECT_ANSWERS_CSV) })?;
    fs::copy(CORRECT_ANSWERS_JSON, PathBuf::from(BACKUP_DIR).join(CORRECT_ANSWERS_JSON))
	.context(CopyFileErr { filename: PathBuf::from(CORRECT_ANSWERS_JSON) })?;
    fs::copy(ALL_QUESTIONS_TEX, PathBuf::from(BACKUP_DIR).join(ALL_QUESTIONS_TEX))
	.context(CopyFileErr { filename: PathBuf::from(ALL_QUESTIONS_TEX) })?;
    fs::copy(LATEX_PREAMBLE, PathBuf::from(BACKUP_DIR).join(LATEX_PREAMBLE))
	.context(CopyFileErr { filename: PathBuf::from(LATEX_PREAMBLE) })?;    

    Ok(())
}
