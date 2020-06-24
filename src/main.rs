extern crate multi;
use multi::create_profile;
use multi::create_papers;
use std::io;

fn main() -> io::Result<()> {
    create_profile("/home/tgaref/programming/rust/multi/example/questions.json")?;
    create_papers("/home/tgaref/programming/rust/multi/example/questions.json")
    
}

