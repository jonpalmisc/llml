use crate::ast::Node;

use std::collections::HashMap;

// Global TODO:
//  - Handlers should take a vector of string arguments rather than a node
//  - Define macros/helpers for argument checking
//  - Remove "should replace" tuple member (just return Node)
//  - Rename "use" macro to "sub"
//  - Add proper error handling to eval()
//  - So much more

type MacroHandler = fn(&mut Context, &Node) -> Node;

fn macro_def(context: &mut Context, node: &Node) -> Node {
    if let Node::MacroCall(_, args) = node {
        if let Node::Literal(k) = &args[0] {
            if let Node::Literal(v) = &args[1] {
                context.vars.insert(k.to_string(), v.to_string());
            }
        }
    }

    Node::Null
}

fn macro_use(context: &mut Context, node: &Node) -> Node {
    let mut value = String::from("???");

    if let Node::MacroCall(_, args) = node {
        if let Node::Literal(k) = &args[0] {
            value = match context.vars.get(k) {
                Some(s) => s.to_string(),
                None => "???".to_string(),
            };
        }
    }

    Node::Literal(value)
}

/// An evaluation context.
pub struct Context {
    pub vars: HashMap<String, String>,
    pub macros: HashMap<String, MacroHandler>,
}

impl Context {
    fn find_macro(&self, name: &str) -> Result<&MacroHandler, String> {
        self.macros
            .get(name)
            .ok_or(format!("Cannot call unregistered macro '{}'", name))
    }

    /// Create a new empty evaluation context.
    pub fn new() -> Self {
        Context {
            vars: HashMap::new(),
            macros: HashMap::new(),
        }
    }

    /// Register the default macros for this context.
    pub fn register_defaults(&mut self) {
        self.macros.insert("def".to_string(), macro_def);
        self.macros.insert("use".to_string(), macro_use);
    }

    /// Evaluate a MacroCall node and get the result.
    fn call(&mut self, node: &mut Node) -> Result<Node, String> {
        if let Node::MacroCall(name, _) = node {
            let handler = self.find_macro(&name)?;
            Ok(handler(self, node))
        } else {
            Err("Tried to call non-macro node".to_string())
        }
    }

    /// Evaluate the given node under the current context.
    pub fn eval(&mut self, node: &mut Node) -> Result<(), String> {
        match node {
            Node::Root(c) | Node::Element(_, c) => {
                for d in c {
                    self.eval(d)?;
                }
            }
            Node::MacroCall(..) => {
                *node = self.call(node)?;
            }
            _ => (),
        }

        Ok(())
    }
}
