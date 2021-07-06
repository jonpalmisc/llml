#[macro_use]
extern crate pest_derive;
extern crate pest;

mod parser;
mod tree;

use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("LLML")
        .version("0.1.0")
        .author("Jon Palmisciano <jp@jonpalmisc.com")
        .arg(
            Arg::new("INPUT")
                .about("Path to the LLML file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .about("Print debug messages and info"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .about("Show help and usage information"),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .about("Show the program version"),
        )
        .get_matches();

    let input_path = matches.value_of("INPUT").unwrap();

    let file_content = fs::read_to_string(input_path).unwrap();
    let tree = tree::Node::from_file_content(&file_content);

    tree.html_print(0);
}
