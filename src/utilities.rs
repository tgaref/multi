use super::*;

pub fn backup<P>(questions: P) -> Result<()>
where
    P: AsRef<Path>
{
    if ! Path::new(BACKUP_DIR).exists() {
	fs::create_dir(BACKUP_DIR).expect("Failed to create backup directory");
    }
    let filenames = vec![questions.as_ref(),
			  Path::new(EXAM_PROFILE_JSON),
			  Path::new(MARK_PROFILE_JSON),
			  Path::new(TEST_PAPERS_JSON),
			  Path::new(TEST_PAPERS_TEX),
			  Path::new(CORRECT_ANSWERS_CSV),
			  Path::new(CORRECT_ANSWERS_JSON),
			  Path::new(ALL_QUESTIONS_TEX),
			  Path::new(LATEX_PREAMBLE)];

    for f in filenames.into_iter().filter(|path| path.exists()) {
	fs::copy(&f, PathBuf::from(BACKUP_DIR).join(&f))
	.context(CopyFileErr { filename: f.to_path_buf() })?;
    }
    Ok(())
}
