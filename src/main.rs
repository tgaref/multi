extern crate multi;
use multi::{parse_arguments, create_papers, create_profile, mark};
use multi::{Config};

fn main() {
    let config = parse_arguments();
    run(config);    
}

fn run(config: Config) -> () {
    if config.cmd == "setup" {
	match create_profile(&config.filename) {
	    Ok(()) => (),
	    Err(e) => eprintln!("A problem occurred during setup: {}", e)
	}	    
    } else if config.cmd == "create" {
	match create_papers(&config.filename) {
	    Ok(()) => (),
	    Err(e) => eprintln!("A problem occurred during exam create: {}", e)
	}
    } else if config.cmd == "mark" {
	match mark(&config.filename) {
	    Ok(()) => (),
	    Err(e) => eprintln!("A problem occurred during exam mark: {}", e)
	}
    } else {
	println!("Run multi --help");
    }
}
