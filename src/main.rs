use aei_tag_parser::AEITagData;
use atty::Stream;
use clap::{arg, command, value_parser, ArgMatches};
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    process,
};

#[cfg(not(tarpaulin_include))]
fn main() {
    let matches = cli();
    // tags to parse
    let mut tags: Vec<String> = Vec::new();

    // If there is a file specified in the argument, add the contained tags in the list of tags to parse
    if let Some(path) = matches.get_one::<PathBuf>("file") {
        read_tags_from_file(path, &mut tags);
    } else if matches.contains_id("stdin") {
        // Read tags from stdin
        read_tags_from_stdin(&mut tags);
    }

    // Extract the tags passed as argument when calling the program
    read_tags_from_cli(&matches, &mut tags);
    if matches.contains_id("csv") {
        print_tags(&tags, true);
    } else {
        print_tags(&tags, false);
    }
}

#[cfg(not(tarpaulin_include))]
fn read_tags_from_file(path: &PathBuf, out: &mut Vec<String>) {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Couldn't open file : {} because {}",
                path.to_string_lossy(),
                e.to_string()
            );
            process::exit(1);
        }
    };

    let mut lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect();

    out.append(&mut lines);
}

#[cfg(not(tarpaulin_include))]
fn read_tags_from_cli(matches: &ArgMatches, out: &mut Vec<String>) {
    let mut vals: Vec<String> = matches
        .get_many::<String>("tags")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|&s| String::from(s))
        .collect();

    out.append(&mut vals);
}

#[cfg(not(tarpaulin_include))]
fn read_tags_from_stdin(out: &mut Vec<String>) {
    let stdin = io::stdin();
    if atty::isnt(Stream::Stdin) {
        for line in stdin.lock().lines() {
            if let Ok(l) = line {
                out.push(l);
            } else if let Err(e) = line {
                eprintln!("{}", e);
            }
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn print_tags(tags: &Vec<String>, csv: bool) {
    for val in tags {
        let tag = match AEITagData::new(&val) {
            Ok(val) => {
                if csv {
                    val.to_csv()
                } else {
                    val.to_short_string()
                }
            }
            Err(e) => e.to_string(),
        };

        println!("{}", tag);
    }
}

#[cfg(not(tarpaulin_include))]
fn cli() -> clap::ArgMatches {
    command!()
        .arg(
            arg!(-f --file "Path to a file to read the tags from")
                .takes_value(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!([tags] "One or multiple tags to parse")
                .value_parser(value_parser!(String))
                .multiple_values(true),
        )
        .arg(arg!(-s --stdin "Get the data from stdin"))
        .arg(arg!(--csv "Print the data in CSV format"))
        .arg_required_else_help(true)
        .get_matches()
}
