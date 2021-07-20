use crate::ast::Node;

use std::collections::HashMap;

// Global TODO:
//  - Handlers should take a vector of string arguments rather than a node
//  - Define macros/helpers for argument checking
//  - Remove "should replace" tuple member (just return Node)
//  - Rename "use" macro to "sub"
//  - Add proper error handling to eval()
//  - So much more

type MacroHandler = fn(&mut Context, &Node) -> (bool, Node);
type EvalResult = (bool, Node);

fn macro_def(context: &mut Context, node: &Node) -> EvalResult {
    if let Node::MacroCall(_, args) = node {
        if let Node::Literal(k) = &args[0] {
            if let Node::Literal(v) = &args[1] {
                context.vars.insert(k.to_string(), v.to_string());
            }
        }
    }

    (true, Node::Null)
}

fn macro_use(context: &mut Context, node: &Node) -> EvalResult {
    let mut value = String::from("???");

    if let Node::MacroCall(_, args) = node {
        if let Node::Literal(k) = &args[0] {
            value = match context.vars.get(k) {
                Some(s) => s.to_string(),
                None => "???".to_string(),
            };
        }
    }

    (true, Node::Literal(value))
}

/// An evaluation context.
pub struct Context {
    pub vars: HashMap<String, String>,
    pub macros: HashMap<String, MacroHandler>,
}

impl Context {
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

    fn call(&mut self, node: &mut Node) -> Result<EvalResult, String> {
        if let Node::MacroCall(name, _) = node {
            if !self.macros.contains_key(name) {
                return Err(format!("Attempted to call unregistered macro '{}'", name));
            }

            let handler = self.macros.get(name).unwrap();

            Ok(handler(self, node))
        } else {
            return Err("Tried to call non-macro node".to_string());
        }
    }

    /// Evaluate the given node under the current context.
    pub fn eval(&mut self, node: &mut Node) {
        match node {
            Node::Root(c) | Node::Element(_, c) => {
                for d in c {
                    self.eval(d);
                }
            }
            Node::MacroCall(..) => {
                let result = self.call(node).unwrap();
                if result.0 {
                    *node = result.1;
                }
            }
            _ => (),
        }
    }
}
