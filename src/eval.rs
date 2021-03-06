use crate::ast::Node;

use std::collections::HashMap;

type MacroArgs = [Node];
type MacroHandler = fn(&mut Context, &MacroArgs) -> Result<Node, String>;

/// Built-in macro to define a new user macro.
fn builtin_defmacro(context: &mut Context, args: &MacroArgs) -> Result<Node, String> {
    let name = args[0].string_value().unwrap();
    let template = args[2].clone();

    context.macros.insert(name.clone(), template);

    Ok(Node::Consumed(format!("defmacro/{}", name)))
}

/// Built-in macro to insert a macro argument.
fn builtin_arg(context: &mut Context, args: &MacroArgs) -> Result<Node, String> {
    let macro_args = context.arg_stack.last().unwrap();
    let index: usize = args[0].string_value().unwrap().parse().unwrap();

    let mut arg = macro_args[index - 1].clone();
    context.eval(&mut arg)?;

    Ok(arg)
}

/// Built-in macro to define a new variable.
fn builtin_def(context: &mut Context, args: &MacroArgs) -> Result<Node, String> {
    let name = args[0].string_value().unwrap();
    let value = args[1].clone();

    context.vars.insert(name.clone(), value);

    Ok(Node::Consumed(name))
}

/// Built-in macro to insert a variable's content.
fn builtin_sub(context: &mut Context, args: &MacroArgs) -> Result<Node, String> {
    let name = args[0].string_value().unwrap();

    Ok(match context.vars.get(&name) {
        Some(v) => v.clone(),
        None => Node::Null,
    })
}

/// An evaluation context.
pub struct Context {
    pub builtins: HashMap<String, MacroHandler>,
    pub macros: HashMap<String, Node>,
    pub vars: HashMap<String, Node>,
    pub arg_stack: Vec<Vec<Node>>,
}

impl Context {
    /// Create a new empty evaluation context.
    pub fn new() -> Self {
        Context {
            builtins: HashMap::new(),
            macros: HashMap::new(),
            vars: HashMap::new(),
            arg_stack: Vec::new(),
        }
    }

    /// Register the default macros for this context.
    pub fn register_defaults(&mut self) {
        self.builtins
            .insert("defmacro".to_string(), builtin_defmacro);
        self.builtins.insert("arg".to_string(), builtin_arg);
        self.builtins.insert("def".to_string(), builtin_def);
        self.builtins.insert("sub".to_string(), builtin_sub);
    }

    /// Evaluate a MacroCall node and get the result.
    fn call(&mut self, name: &str, args: &MacroArgs) -> Result<Node, String> {
        if let Some(builtin_handler) = self.find_builtin(name) {
            builtin_handler(self, args)
        } else if let Some(macro_template) = self.find_macro(name) {
            let mut working_copy = macro_template.clone();

            self.arg_stack.push(args.to_vec());
            self.eval(&mut working_copy)?;
            self.simplify(&mut working_copy)?;
            self.arg_stack.pop();

            Ok(working_copy)
        } else {
            Err(format!("Tried to call undefined macro '{}'", name))
        }
    }

    /// Evaluate the given node under the current context.
    pub fn eval(&mut self, node: &mut Node) -> Result<(), String> {
        match node {
            Node::Root(c) | Node::Element(_, c) | Node::Wrapper(c) => {
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

    /// Quite possibly the least efficient function of all time.
    pub fn simplify(&mut self, node: &mut Node) -> Result<(), String> {
        match node {
            Node::Root(c) | Node::Element(_, c) | Node::Wrapper(c) => {
                let mut new_children = Vec::new();

                for d in c.clone() {
                    match d {
                        Node::Wrapper(mut w) => new_children.append(&mut w),
                        _ => new_children.push(d.clone()),
                    }
                }

                for mut x in &mut new_children {
                    self.simplify(&mut x)?;
                }

                *c = new_children;
            }
            _ => (),
        }

        Ok(())
    }

    /// Find a built-in macro by name.
    fn find_builtin(&self, name: &str) -> Option<&MacroHandler> {
        self.builtins.get(name)
    }

    /// Find a user macro by name.
    fn find_macro(&self, name: &str) -> Option<&Node> {
        self.macros.get(name)
    }
}
