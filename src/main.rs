#[macro_use]
extern crate pest_derive;
extern crate pest;

use std::fs;

mod parser;
mod tree;
use crate::parser::LtmlParser;
use crate::tree::TreeBuilder;

fn main() {
    let file_content = fs::read_to_string("test.ltml").unwrap();
    let file = LtmlParser::parse_file_content(&file_content);

    let tree = TreeBuilder::from_parsed_file(file);
    tree.print(0);
}
