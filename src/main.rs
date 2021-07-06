#[macro_use]
extern crate pest_derive;
extern crate pest;

use std::fs;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LtmlParser;

enum TreeNodeType {
    Element(String),
    Literal(String),
}

struct TreeNode {
    kind: TreeNodeType,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn from_literal(pair: Pair<Rule>) -> Self {
        TreeNode {
            kind: TreeNodeType::Literal(pair.as_str().to_string()),
            children: vec![],
        }
    }

    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut node = TreeNode {
            kind: TreeNodeType::Element("NULL".to_string()),
            children: vec![],
        };

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::ElementName => node.kind = TreeNodeType::Element(p.as_str().to_string()),
                Rule::Element => node.children.push(TreeNode::from_pair(p)),
                Rule::Literal => node.children.push(TreeNode::from_literal(p)),
                _ => (),
            }
        }

        return node;
    }

    pub fn from_file(pair: Pair<Rule>) -> Self {
        if pair.as_rule() != Rule::File {
            panic!("Expected file rule");
        }

        let mut children: Vec<TreeNode> = vec![];
        for p in pair.into_inner() {
            children.push(TreeNode::from_pair(p));
        }

        TreeNode {
            kind: TreeNodeType::Element("ROOT".to_string()),
            children: children,
        }
    }

    fn internal_print(&self, level: usize) {
        match &self.kind {
            TreeNodeType::Element(t) => {
                println!(
                    "{}Element<{}, {}>",
                    " ".repeat(level * 2),
                    t,
                    self.children.len()
                );

                for c in &self.children {
                    c.internal_print(level + 1);
                }

                println!("");
            }
            TreeNodeType::Literal(s) => {
                println!("{}Literal<{:?}>", " ".repeat(level * 2), s);
            }
        }
    }

    pub fn print(&self) {
        for c in &self.children {
            c.internal_print(0);
        }
    }
}

fn main() {
    let raw_input = fs::read_to_string("test.ltml").expect("Couldn't read input");
    let file = LtmlParser::parse(Rule::File, &raw_input)
        .unwrap_or_else(|e| panic!("Failed to parse input\n{:#?}", e))
        .next()
        .unwrap();

    let tree = TreeNode::from_file(file);
    tree.print();
}
