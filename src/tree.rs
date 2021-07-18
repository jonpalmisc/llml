use core::fmt::{self, Write};

use crate::parser::{LlmlParser, Rule};
use pest::iterators::Pair;

type ChildNodes = Vec<Node>;

// AST nodes don't map 1:1 to DOM elements - but some nodes do.
pub enum Node {
    Root(ChildNodes),
    Element(String, ChildNodes),
    Attribute(String, String),
    Literal(String),
    Null,
}

impl Node {
    /// Create a new Literal node from a string.
    fn new_literal(content: &str) -> Self {
        Node::Literal(content.to_string())
    }

    /// Create a new Literal node from a Literal rule.
    fn from_literal_rule(pair: Pair<Rule>) -> Self {
        Self::new_literal(pair.as_str())
    }

    /// Create a new Attribute node with the given key and value.
    fn new_attribute(key: &str, value: &str) -> Self {
        Self::Attribute(key.to_string(), value.to_string())
    }

    /// Create a new Attribute node from an Attribute rule.
    fn from_attribute_rule(pair: Pair<Rule>) -> Self {
        let mut ar = pair.into_inner();
        let key = ar.next().unwrap().as_str();
        let value = ar.next().unwrap().as_str();

        Self::new_attribute(key, value)
    }

    /// Create a new Attribute node from an ElementClass rule.
    fn from_element_class_rule(pair: Pair<Rule>) -> Self {
        let name = pair.as_str().replace(".", "");
        Self::new_attribute("class", &name)
    }

    /// Create a new node from an Element rule.
    fn from_element_rule(pair: Pair<Rule>) -> Self {
        let mut name = String::new();
        let mut children: Vec<Node> = vec![];

        for p in pair.into_inner() {
            let el = match p.as_rule() {
                Rule::ElementName => {
                    name = p.as_str().to_string();
                    Self::Null
                }
                Rule::ElementClass => Self::from_element_class_rule(p),
                Rule::Attribute => Self::from_attribute_rule(p),
                Rule::Element => Self::from_element_rule(p),
                Rule::Literal => Self::from_literal_rule(p),
                _ => unreachable!(),
            };

            match el {
                Self::Null => (),
                _ => children.push(el),
            }
        }

        Node::Element(name, children)
    }

    /// Create a node tree from a File rule generated by the parser.
    pub fn from_parsed_file(pair: Pair<Rule>) -> Self {
        if pair.as_rule() != Rule::File {
            panic!("Expected file rule");
        }

        let mut children: Vec<Node> = vec![];
        for p in pair.into_inner() {
            let e = Self::from_element_rule(p);
            match e {
                Self::Null => continue,
                Self::Element(ref n, _) => {
                    if n.is_empty() {
                        continue;
                    }
                }
                _ => (),
            }

            children.push(e);
        }

        Node::Root(children)
    }

    /// Create a node tree from LLML input.
    pub fn from_file_content(content: &str) -> Self {
        let parsed_file = LlmlParser::parse_file_content(content);
        Self::from_parsed_file(parsed_file)
    }
}

/// Pretty-print a vector of AST nodes.
fn write_nodes(f: &mut fmt::Formatter, nodes: &Vec<Node>) -> fmt::Result {
    use indenter::indented;
    let mut df = indented(f).with_str("  ");

    for (i, c) in nodes.iter().enumerate() {
        writeln!(df, "{}.{}", i, c)?;
    }

    Ok(())
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Node::*;

        match self {
            Null => write!(f, "Null<>"),
            Literal(s) => write!(f, "Literal[{:?}],", s),
            Attribute(k, v) => write!(f, "Attribute[{}={:?}],", k, v),
            Element(n, c) => {
                writeln!(f, "Element[{}] {{", n)?;
                write_nodes(f, c)?;
                write!(f, "}},")
            }
            Root(r) => {
                writeln!(f, "Root {{")?;
                write_nodes(f, r)?;
                write!(f, "}}")
            }
        }
    }
}
