use std::collections::HashMap;
use std::fmt;

use crate::ast::Node;

/// A singular HTML tag.
struct Tag {
    name: String,
    attributes: HashMap<String, String>,
    content: String,
}

impl Tag {
    /// Create a new empty tag.
    pub fn new() -> Self {
        Tag {
            name: String::from("null"),
            attributes: HashMap::new(),
            content: String::new(),
        }
    }

    /// Set the tag's name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Add an attribute to the tag.
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Append text content to the tag's body.
    pub fn append_content(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>{}</{}>", self.name, self.content, self.name)
    }
}

/// Serialize an AST node to HTML.
pub fn serialize_node(node: Node) -> Result<String, String> {
    let mut tag = Tag::new();

    let children = match node {
        Node::Root(r) => {
            tag.set_name("html");
            r
        }
        Node::Element(n, c) => {
            tag.set_name(&n);
            c
        }
        Node::Null => return Ok("".to_string()),
        _ => unreachable!(),
    };

    for c in children {
        match c {
            Node::Element(..) => {
                tag.append_content(&serialize_node(c)?);
            }
            Node::Attribute(k, v) => {
                tag.add_attribute(&k, &v);
            }
            Node::Literal(l) => {
                tag.append_content(&l);
            }
            Node::MacroCall(..) => tag.append_content("MACRO_CALL"),
            Node::Null => (),
            _ => unreachable!(),
        }
    }

    Ok(format!("{}", tag))
}
