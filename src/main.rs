#[macro_use]
extern crate pest_derive;
extern crate pest;

mod ast;
mod eval;
mod html;
mod parser;

use clap::{App, AppSettings, Arg};
use std::fs::File;
use std::io::Write;
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
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Where to write the HTML output"),
        )
        .help_message("Show this help information")
        .version_message("Print the program version")
        .get_matches();

    // This unwrap() should never fail since it's a required argument.
    let input_path = matches.value_of("INPUT").unwrap();
    if !input_path.ends_with(".llml") {
        return Err("Input files must have the .llml extension".to_string());
    }

    // Attempt to read from the given input path.
    let file_content = fs::read_to_string(input_path)
        .map_err(|_| format!("Failed to read input file '{}'", input_path))?;

    let parse_start = time::Instant::now();
    let mut tree = ast::Node::from_string(&file_content)?;
    let parse_span = parse_start.elapsed();

    // Print the AST if requested.
    if matches.is_present("tree") {
        println!("Parsed syntax tree:\n");
        println!("{}\n", format!("  {}", tree).replace("\n", "\n  "));
    }

    // Evaluate the raw AST.
    let eval_start = time::Instant::now();
    let mut context = eval::Context::new();
    context.register_defaults();
    context.eval(&mut tree)?;
    let eval_span = eval_start.elapsed();

    // Print the evaluated AST if requested.
    if matches.is_present("tree") {
        println!("Evaluated syntax tree:\n");
        println!("{}\n", format!("  {}", tree).replace("\n", "\n  "));
    }

    let serialize_start = time::Instant::now();
    let html = html::serialize_node(tree)?;
    let serialize_span = serialize_start.elapsed();

    // Print performance info if requested.
    if matches.is_present("profile") {
        println!("Performance info:\n");
        println!("  * Input parsed to AST in {:?}", parse_span);
        println!("  * Macros evaluated in {:?}", eval_span);
        println!("  * AST serialized to HTML in {:?}", serialize_span);
    }

    // Write the resulting HTML.
    let output_path = match matches.value_of("output") {
        Some(p) => p.to_string(),
        None => input_path.replace(".llml", ".html"),
    };

    // Create and write the output file.
    let mut output_file = File::create(output_path.clone())
        .map_err(|_| format!("Failed to create output file '{}'", output_path))?;
    output_file
        .write_all(html.as_bytes())
        .map_err(|_| format!("Failed to write output file '{}'", output_path))?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        process::exit(1);
    }
}
