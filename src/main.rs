use clap::{Command, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = Command::new("greppy")
        .version("0.1")
        .about("A little program to search things")
        .arg(Arg::new("pattern")
            .help("The pattern to search for.")
            .required(true))
        .arg(Arg::new("file")
            .help("The file to search in")
            .required(true))
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let file = args.get_one::<String>("file").unwrap();

    let regex = Regex::new(pattern).unwrap();

    let file_to_read = File::open(file).unwrap();

    let reader = BufReader::new(file_to_read);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match regex.find(&line) {
            Some(_) => println!("{}", line),
            None => ()
        }
    }
}
