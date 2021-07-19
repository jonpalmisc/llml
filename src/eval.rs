use crate::ast::Node;

use std::collections::HashMap;

/// An evaluation context.
pub struct Context {
    vars: HashMap<String, String>,
}

impl Context {
    /// Create a new empty evaluation context.
    pub fn new() -> Self {
        Context {
            vars: HashMap::new(),
        }
    }

    /// Evaluate the given tree under the current context.
    pub fn eval(&self, tree: &mut Node) {}
}
