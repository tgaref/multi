use super::*;
use std::string::String;

pub fn write_all_questions<P>(exam: &Exam, filename: P) -> Result<()>
where
    P: AsRef<Path> + Display
{
    let filename = filename.as_ref();
    let mut groups: HashMap<String, Vec<Question>> = HashMap::new();
    for question in &exam.questions {
	if let Some(v) = groups.get_mut(&question.group) {
	    v.push(question.clone());
	} else {
	    groups.insert(question.group.clone(), vec![question.clone()]);
	}
    }
       
    let file = OpenOptions::new()
	.write(true)
	.create(true)
	.open(filename)
	.context( OpenFileErr { filename: filename.to_path_buf() })?;
    let mut file = BufWriter::new(file);

    let preamble = fs::read(LATEX_PREAMBLE)
	.context(OpenFileErr {filename: filename.to_path_buf()})?;
    file.write(&preamble).context( SaveFileErr { filename: filename.to_path_buf() })?;

    writeln!(file, "\\title{{ {} }}",exam.name).context(SaveFileErr {filename: filename.to_path_buf()})?;
    writeln!(file, "\\pagestyle{{empty}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
    writeln!(file, "\\begin{{document}}" ).context(SaveFileErr {filename: filename.to_path_buf()})?;
    writeln!(file, "\\maketitle").context(SaveFileErr {filename: filename.to_path_buf()})?;

    for (group, questions) in &groups {
	writeln!(file, "\\flushleft \\underline{{\\bf {} }}",group).context(SaveFileErr {filename: filename.to_path_buf()})?;
	writeln!(file, "\\begin{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
	for question in questions {
	    writeln!(file, "\\item {}", question.statement).context(SaveFileErr {filename: filename.to_path_buf()})?;
	    writeln!(file, "\\begin{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
	    for answer in &question.answers {
		writeln!(file, "\\item ({})  {}", answer.correct, answer.ans).context(SaveFileErr {filename: filename.to_path_buf()})?;
	    }
	    writeln!(file,"\\end{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
	}
	writeln!(file,"\\end{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
    }
    writeln!(file,"\\end{{document}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
    Ok(())
}

pub fn write_test_papers<P>(exam: &Exam, test_papers: &Vec<Paper>, exam_profile: &ExamProfile, filename: P) -> Result<()>
where
    P: AsRef<Path> + Display
{
    let filename = filename.as_ref();
    // Create a map group -> save_space based on exam_profile
    let mut save_space_map = HashMap::new();
    for group_prof in &exam_profile.profile {
	save_space_map.insert(group_prof.group.clone(), group_prof.save_space);
    }

    let file = OpenOptions::new()
	.write(true)
	.create(true)
	.open(filename)
	.context(OpenFileErr { filename: filename.to_path_buf() })?;
    let mut file = BufWriter::new(file);

    let preamble = fs::read(LATEX_PREAMBLE)
	.context(OpenFileErr {filename: filename.to_path_buf()})?;
    file.write(&preamble)
	.context(SaveFileErr {filename: filename.to_path_buf()})?;
    
    writeln!(file, "\\usepackage{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
    writeln!(file, "\\pagestyle{{empty}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
    writeln!(file, "\\begin{{document}}").context(SaveFileErr {filename: filename.to_path_buf()})?;

    for paper in test_papers {
	let num_of_questions = paper.questions.len();
        let ans_per_line = if num_of_questions > 10 { 8 } else { num_of_questions };
        let num_of_lines = (num_of_questions / ans_per_line) + if num_of_questions % ans_per_line == 0 { 0 } else { 1 };
        writeln!(file, "{{\\Large\\bf {} }} \\hspace{{1.5cm}}", paper.serial)
	    .context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\begin{{tabular}}{{| {} }}", std::iter::repeat("l|").take(ans_per_line).collect::<String>())
	    .context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\hline").context(SaveFileErr {filename: filename.to_path_buf()})?;
	for l in 0..num_of_lines {
	    writeln!(file, "{{\\large {} }}: \\hspace*{{0.5cm}}", l * ans_per_line + 1)
		.context(SaveFileErr {filename: filename.to_path_buf()})?;
	    for i in 2..=ans_per_line {
		if l * ans_per_line + i <= num_of_questions {
		    writeln!(file, "& {{\\large {} }}: \\hspace*{{0.5cm}}", l * ans_per_line + i)
			.context(SaveFileErr {filename: filename.to_path_buf()})?;
		} else {
		    writeln!(file, "& ").context(SaveFileErr {filename: filename.to_path_buf()})?;
		}
	    }
	    writeln!(file, "\\\\ \n \\hline").context(SaveFileErr {filename: filename.to_path_buf()})?;
	}

	writeln!(file, "\\end{{tabular}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\vspace*{{1cm}} \n").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "{{\\flushleft Κατεύθυνση: }}").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "{{\\flushleft Όνομα/Α.Μ.: }}").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\vspace*{{0.5cm}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\begin{{center}} {{\\Large {} }} \\end{{center}}", exam.name)
	    .context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\begin{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
	for question in &paper.questions {
	    writeln!(file, "\\item {}", question.statement).context(SaveFileErr {filename: filename.to_path_buf()})?;
	    if save_space_map[&question.group] {
		writeln!(file, " ").context(SaveFileErr {filename: filename.to_path_buf()})?;
		for (answer,i) in question.answers.iter().zip(1..) {
		    writeln!(file, "({}) {} \\hspace{{1.5cm}} ", i, answer.ans).context(SaveFileErr {filename: filename.to_path_buf()})?;
		}
		writeln!(file, " ").context(SaveFileErr {filename: filename.to_path_buf()})?;
	    } else {
		writeln!(file, "\\begin{{enumerate}}[(1)]").context(SaveFileErr {filename: filename.to_path_buf()})?;
		for answer in &question.answers {
		    writeln!(file, "    \\item {}", answer.ans).context(SaveFileErr {filename: filename.to_path_buf()})?;
		}
		writeln!(file, "\\end{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
	    }
	}
        writeln!(file, "\\end{{enumerate}}").context(SaveFileErr {filename: filename.to_path_buf()})?;
        writeln!(file, "\\hrulefill \\\\ \n").context(SaveFileErr {filename: filename.to_path_buf()})?;
	writeln!(file, "{} \n", exam.footer).context(SaveFileErr {filename: filename.to_path_buf()})?; 
        writeln!(file, "\\newpage \n").context(SaveFileErr {filename: filename.to_path_buf()})?;
    }
    writeln!(file, "\\end{{document}}").context(SaveFileErr {filename: filename.to_path_buf()})?;

    Ok(())
}
