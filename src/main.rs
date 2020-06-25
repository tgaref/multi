extern crate multi;
use multi::create_profile;
use multi::create_papers;
use multi::mark;
//use std::error::Error;

fn main() {
    create_profile("/home/tgaref/programming/rust/multi/example/questions.json").expect("Problem in create profile");
    create_papers("/home/tgaref/programming/rust/multi/example/questions.json").expect("Problem in create papers");
    mark("/home/tgaref/programming/rust/multi/example/given_answers.csv").expect("Problem in mark");
	
    
}

