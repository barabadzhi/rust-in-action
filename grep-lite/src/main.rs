extern crate clap;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use clap::{App, Arg};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();

        process_lines(reader, re);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);

        process_lines(reader, re);
    }
}
