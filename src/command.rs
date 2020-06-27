extern crate clap;
use clap::{App, Arg, SubCommand};


pub struct Config<'a> {
    pub cmd: &'a str,
    pub filename: String
}

impl <'a> Config<'a> {
    pub fn new(cmd: &'a str, filename: String) -> Config<'a> {
	Config { cmd, filename }
    }
}


pub fn parse_arguments<'a>() -> Config<'a> {
    let matches = App::new("Mutli: Create and mark multiple choice tests")
        .version("0.1")
        .author("Theo G. <tgaref@gmail.com>")
        .about("A utility for creating and marking multiple choice tests")
	.subcommand(
	    SubCommand::with_name("setup")
		.about("Create exam and mark profile files")
		.arg(
		    Arg::with_name("questions1")
			.help("The JSON file with all the questions")
			.required(true)
			.index(1),
		)		
	)
	.subcommand(
	    SubCommand::with_name("create")
		.about("Create exam files")
		.arg(
		    Arg::with_name("questions2")
			.help("The JSON file with all the questions")
			.required(true)
			.index(1),
		)		
	)
	.subcommand(
	    SubCommand::with_name("mark")
		.about("Mark given answers")
		.arg(
		    Arg::with_name("answers")
			.help("The CSV file with given answers")
			.required(true)
			.index(1),
		)		
	)
	.subcommand(
	    SubCommand::with_name("backup")
		.about("Backup exam files")
		.arg(
		    Arg::with_name("questions")
			.help("Backup exam files")
			.required(true)
			.index(1),
		)		
	)

        .get_matches();

    match matches.subcommand() {
	("setup", Some(sub))  => {	    
	    let filename = sub.value_of("questions1").unwrap();
	    Config::new("setup", filename.to_string())
	}, 
	("create", Some(sub)) => {
	    let filename = sub.value_of("questions2").unwrap();
	    Config::new("create", filename.to_string())
	},
	("mark", Some(sub))   => {
	    let filename = sub.value_of("answers").unwrap();
	    Config::new("mark", filename.to_string())
	},
	("backup", Some(sub)) => {
	    let filename = sub.value_of("questions").unwrap();
	    Config::new("backup", filename.to_string())
	},
	_                => {
	    Config::new("", "".to_string())
	}
    }
}
