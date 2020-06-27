use std::io::{self, Write};
extern crate multi;
use multi::{parse_arguments, create_papers, create_profile, mark, backup};
use multi::{Config};

fn main() {
    let config = parse_arguments();
    run(config);    
}

fn run(config: Config) -> () {
    if config.cmd == "setup" {
	match create_profile(&config.filename) {
	    Ok(()) => println!("Done!"),
	    Err(e) => eprintln!("A problem occurred during setup: {}", e)
	}	    
    } else if config.cmd == "create" {
	if std::path::Path::new(multi::TEST_PAPERS_JSON).exists() {
	    print!("An exam already exists. Do you want to replace it? [y/n]: ");
	    io::stdout().flush().unwrap();
	    let mut input = String::new();
	    match io::stdin().read_line(&mut input) {
		Ok(_) => { 
		    if input.trim() == "y" || input.trim() == "yes" {
			match create_papers(&config.filename) {
			    Ok(()) => println!("Done!"),
			    Err(e) => eprintln!("A problem occurred during exam create: {}", e)
			}
		    } else { () }			
		}
		_ => ()
	    }
	} else {
	    match create_papers(&config.filename) {
		Ok(()) => println!("Done!"),
		Err(e) => eprintln!("A problem occurred during exam create: {}", e)
	    }
	}	    
    } else if config.cmd == "mark" {
	match mark(&config.filename) {
	    Ok(()) => println!("Done!"),
	    Err(e) => eprintln!("A problem occurred during exam mark: {}", e)
	}
    } else if config.cmd == "backup" {
	match backup(&config.filename) {
	    Ok(()) => println!("Done!"),
	    Err(e) => eprintln!("A problem occurred during backup: {}", e)
	}	    
    }
    else {
	println!("Run multi --help");
    }
}
