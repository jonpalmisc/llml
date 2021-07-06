use crate::parser::Rule;
use pest::iterators::Pair;

pub enum TreeNode {
    Root(Vec<TreeNode>),
    Element {
        name: String,
        children: Vec<TreeNode>,
    },
    Literal(String),
}

impl TreeNode {
    pub fn print(&self, level: usize) {
        match &self {
            Self::Root(children) => {
                for c in children {
                    c.print(0);
                }
            }
            Self::Element { name, children } => {
                println!(
                    "{}Element<{}, {}>",
                    " ".repeat(level * 2),
                    name,
                    children.len()
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

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::ElementName => name = String::from(p.as_str()),
                Rule::Element => children.push(Self::node_from_pair(p)),
                Rule::Literal => children.push(Self::node_from_literal(p)),
                _ => (),
            }
        }

        TreeNode::Element { name, children }
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
}
