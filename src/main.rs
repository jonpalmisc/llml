#[macro_use]
extern crate pest_derive;
extern crate pest;

mod parser;
mod tree;

use clap::{App, AppSettings, Arg};
use std::{fs, time};

fn main() -> Result<(), String> {
    let matches = App::new("LLML")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .version("0.1.0")
        .author("Jon Palmisciano <jp@jonpalmisc.com")
        .arg(
            Arg::with_name("INPUT")
                .help("Path to the LLML file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("tree")
                .short("T")
                .long("tree")
                .help("Print the syntax tree after parsing")
        )
        .arg(
            Arg::with_name("profile")
                .short("P")
                .long("profile")
                .help("Display performance info upon exit"),
        )
        .help_message("Show this help information")
        .version_message("Print the program version")
        .get_matches();

    // This unwrap() should never fail since it's a required argument.
    let input_path = matches.value_of("INPUT").unwrap();

    // Attempt to read from the given input path.
    let file_content = fs::read_to_string(input_path)
        .map_err(|_| String::from("Failed to read file at the path provided"))?;

    let parse_start = time::Instant::now();
    let tree = tree::Node::from_file_content(&file_content);
    let parse_span = parse_start.elapsed();

    // Print the AST if requested.
    if matches.is_present("tree") {
        println!("{}", tree);
    }

    // Print performance info if requested.
    if matches.is_present("profile") {
        println!("\n * Input parsed to AST in {:?}", parse_span);
    }

    Ok(())
}
