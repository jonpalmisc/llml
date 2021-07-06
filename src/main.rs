#[macro_use]
extern crate pest_derive;
extern crate pest;

mod parser;
mod tree;

use crate::tree::TreeBuilder;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("test.ltml").unwrap();
    let tree = TreeBuilder::from_file_content(&file_content);
    tree.print(0);
}
