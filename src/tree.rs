use std::collections::HashMap;

use crate::parser::{LlmlParser, Rule};
use pest::iterators::Pair;

pub enum TreeNode {
    Root(Vec<TreeNode>),
    Element {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<TreeNode>,
    },
    Literal(String),
}

fn format_attributes(map: &HashMap<String, String>) -> String {
    let mut r = String::from(" ");

    for (k, v) in map {
        r.push_str(&format!("{}=\"{}\" ", k, v));
    }

    return r.trim_end().to_string();
}

impl TreeNode {
    pub fn print(&self, level: usize) {
        match &self {
            Self::Root(children) => {
                for c in children {
                    c.print(0);
                }
            }
            Self::Element {
                name,
                children,
                attributes,
            } => {
                println!(
                    "{}Element<{}/{}{}>",
                    " ".repeat(level * 2),
                    name,
                    children.len(),
                    format_attributes(attributes),
                );

                for c in children {
                    c.print(level + 1);
                }

                println!("");
            }
            Self::Literal(s) => {
                println!("{}Literal<{:?}>", " ".repeat(level * 2), s);
            }
        }
    }
}

pub struct TreeBuilder;

impl TreeBuilder {
    fn node_from_literal(pair: Pair<Rule>) -> TreeNode {
        TreeNode::Literal(String::from(pair.as_str()))
    }

    fn node_from_pair(pair: Pair<Rule>) -> TreeNode {
        let mut name = String::from("NULL");
        let mut children: Vec<TreeNode> = vec![];
        let mut attributes = HashMap::new();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::ElementName => name = String::from(p.as_str()),
                Rule::AttributeList => {
                    for i in p.into_inner() {
                        let mut a = i.into_inner();
                        let k = a.next().unwrap().as_str();
                        let v = a.next().unwrap().as_str();

                        attributes.insert(k.to_string(), v.to_string());
                    }
                }
                Rule::Element => children.push(Self::node_from_pair(p)),
                Rule::Literal => children.push(Self::node_from_literal(p)),
                _ => (),
            }
        }

        TreeNode::Element {
            name,
            attributes,
            children,
        }
    }

    pub fn from_parsed_file(pair: Pair<Rule>) -> TreeNode {
        if pair.as_rule() != Rule::File {
            panic!("Expected file rule");
        }

        let mut children: Vec<TreeNode> = vec![];
        for p in pair.into_inner() {
            children.push(Self::node_from_pair(p));
        }

        TreeNode::Root(children)
    }

    pub fn from_file_content(content: &str) -> TreeNode {
        let parsed_file = LlmlParser::parse_file_content(content);
        Self::from_parsed_file(parsed_file)
    }
}
