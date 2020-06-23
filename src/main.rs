extern crate multi;
use multi::create_profile;
use std::io;

fn main() -> io::Result<()> {
    create_profile("/home/tgaref/programming/rust/multi/example/questions.json")
}

