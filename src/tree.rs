use std::collections::HashMap;

use crate::parser::{LlmlParser, Rule};
use pest::iterators::Pair;

type NodeList = Vec<Node>;
type AttributeMap = HashMap<String, String>;

pub struct Element {
    name: String,
    attributes: AttributeMap,
    children: NodeList,
}

impl Element {
    pub fn new() -> Self {
        Element {
            name: "NULL".to_string(),
            attributes: HashMap::new(),
            children: vec![],
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

pub enum Node {
    Root(NodeList),
    Element(Element),
    Literal(String),
}

fn format_attributes(map: &HashMap<String, String>) -> String {
    let mut r = String::from(" ");

    for (k, v) in map {
        r.push_str(&format!("{}=\"{}\" ", k, v));
    }

    return r.trim_end().to_string();
}

impl Node {
    pub fn print(&self, level: usize) {
        match &self {
            Self::Root(nodes) => {
                for n in nodes {
                    n.print(0);
                }
            }
            Self::Element(el) => {
                println!(
                    "{}Element<{}/{}{}>",
                    " ".repeat(level * 2),
                    el.name,
                    el.children.len(),
                    format_attributes(&el.attributes),
                );

                for c in &el.children {
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

pub struct Builder;

impl Builder {
    fn node_from_literal(pair: Pair<Rule>) -> Node {
        Node::Literal(String::from(pair.as_str()))
    }

    fn node_from_pair(pair: Pair<Rule>) -> Node {
        let mut el = Element::new();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::ElementName => el.set_name(p.as_str()),
                Rule::AttributeList => {
                    for i in p.into_inner() {
                        let mut a = i.into_inner();
                        let k = a.next().unwrap().as_str();
                        let v = a.next().unwrap().as_str();

                        el.set_attribute(k, v);
                    }
                }
                Rule::Element => el.add_child(Self::node_from_pair(p)),
                Rule::Literal => el.add_child(Self::node_from_literal(p)),
                _ => (),
            }
        }

        Node::Element(el)
    }

    pub fn from_parsed_file(pair: Pair<Rule>) -> Node {
        if pair.as_rule() != Rule::File {
            panic!("Expected file rule");
        }

        let mut children: Vec<Node> = vec![];
        for p in pair.into_inner() {
            children.push(Self::node_from_pair(p));
        }

        Node::Root(children)
    }

    pub fn from_file_content(content: &str) -> Node {
        let parsed_file = LlmlParser::parse_file_content(content);
        Self::from_parsed_file(parsed_file)
    }
}
