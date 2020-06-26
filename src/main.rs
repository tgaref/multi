extern crate multi;
use multi::{Config, parse_arguments};
use multi::create_profile;
use multi::create_papers;
use multi::mark;
//use std::error::Error;

fn main() {
    let config = parse_arguments();
    run(config);    
}


fn run(config: Config) -> () {
    if config.cmd == "setup" {
	create_profile(&config.filename)
	    .expect("Problem in create profile");
    } else if config.cmd == "create" {
	create_papers(&config.filename)
	    .expect("Problem in create papers")
    } else if config.cmd == "mark" {
	mark(&config.filename)
	    .expect("Problem in mark");
    } else {
	println!("Run multi --help");
    }
}
