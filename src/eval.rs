use crate::ast::Node;

use std::collections::HashMap;

// Global TODO:
//  - Define macros/helpers for argument checking
//  - Rename "use" macro to "sub"
//  - So much more

type MacroArgs = [Node];
type MacroHandler = fn(&mut Context, &MacroArgs) -> Node;

fn macro_def(context: &mut Context, args: &MacroArgs) -> Node {
    if let Node::Literal(k) = &args[0] {
        if let Node::Literal(v) = &args[1] {
            context.vars.insert(k.to_string(), v.to_string());
        }
    }

    Node::Null
}

fn macro_use(context: &mut Context, args: &MacroArgs) -> Node {
    let mut value = String::from("???");

    if let Node::Literal(k) = &args[0] {
        if let Some(s) = context.vars.get(k) {
            value = s.to_string()
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
    fn call(&mut self, name: &str, args: &MacroArgs) -> Result<Node, String> {
        let handler = self.find_macro(name)?;
        Ok(handler(self, args))
    }

    /// Evaluate the given node under the current context.
    pub fn eval(&mut self, node: &mut Node) -> Result<(), String> {
        match node {
            Node::Root(c) | Node::Element(_, c) => {
                for d in c {
                    self.eval(d)?;
                }
            }
            Node::MacroCall(ref n, ref a) => {
                *node = self.call(n, a)?;
            }
            _ => (),
        }

        Ok(())
    }
}
