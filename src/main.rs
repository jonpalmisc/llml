#[macro_use]
extern crate pest_derive;
extern crate pest;

mod ast;
mod eval;
mod html;
mod parser;

use clap::{App, AppSettings, Arg};
use std::{fs, process, time};

fn run() -> Result<(), String> {
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
                .help("Print the syntax tree after parsing"),
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
    let mut tree = ast::Node::from_file_content(&file_content)?;
    let parse_span = parse_start.elapsed();

    // Print the AST if requested.
    if matches.is_present("tree") {
        println!("<!-- Parsed tree");
        println!("{}", format!("  {}", tree).replace("\n", "\n  "));
        println!("-->");
    }

    // Evaluate the raw AST.
    let eval_start = time::Instant::now();
    let context = eval::Context::new();
    context.eval(&mut tree);
    let eval_span = eval_start.elapsed();

    // Print the evaluated AST if requested.
    if matches.is_present("tree") {
        println!("<!-- Evaluated tree");
        println!("{}", format!("  {}", tree).replace("\n", "\n  "));
        println!("-->");
    }

    let serialize_start = time::Instant::now();
    println!("{}", html::serialize_node(tree)?);
    let serialize_span = serialize_start.elapsed();

    // Print performance info if requested.
    if matches.is_present("profile") {
        println!("<!-- Performance stats");
        println!(" * Input parsed to AST in {:?}", parse_span);
        println!(" * Macros evaluated in {:?}", eval_span);
        println!(" * AST serialized to HTML in {:?}", serialize_span);
        println!("-->");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        process::exit(1);
    }
}
