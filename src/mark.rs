use super::*;
use csv::{Writer, ReaderBuilder};
use std::error::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    serial: usize,
    am: usize,
    answer: String
}
 
pub fn mark(given_answers_csv: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(given_answers_csv)?;

    let s = fs::read_to_string(CORRECT_ANSWERS_JSON)?;
    
    // maps serial to (correct answer, group name)
    let correct_answers_map: HashMap<usize, Vec<(usize,String)>> = serde_json::from_str(&s)
	.expect(&format!("File {} is not in valid json format", CORRECT_ANSWERS_JSON));

    let marks_map: MarkProfile = serde_json::from_str(&fs::read_to_string(MARK_PROFILE_JSON)?)
	.expect(&format!("File {} is not in valid json format", CORRECT_ANSWERS_JSON));

    let mut wrt = Writer::from_path(RESULTS_CSV)
	.expect(&format!("Failed to open file {}", RESULTS_CSV));
    for record in rdr.records() {
	let mut record = record?;
	record.trim();
	let row: Row = record.deserialize(None)?;
	let correct = &correct_answers_map[&row.serial];
	if row.answer.len() != marks_map.len() {
	    panic!("Number of answers for AM: {} in file {} does not match number of questions", row.serial, given_answers_csv);
	}
	let mut mark: f64 = 0.0;
	let mut correct_string = String::new();
	for (a, (b, grp)) in row.answer.chars().zip(correct) {
	    if a == '-' || a == 'x' { continue };
	    if let Some(b) = std::char::from_digit(*b as u32,10) {
		correct_string.push(b);
		if a == b {
		    mark += marks_map[grp].correct_mark;
		} else {
		    mark += marks_map[grp].wrong_mark;
		}
	    }
	}
	wrt.write_record(&[row.am.to_string(), row.serial.to_string(), correct_string, mark.to_string()])
	    .expect(&format!("Failed to write {} file", RESULTS_CSV));
    }
    wrt.flush().expect(&format!("Failed to write {} file", RESULTS_CSV));
    Ok(())
}
