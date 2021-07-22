use pest::{iterators::Pair, Parser};
use regex::Regex;

/// Clean up input before parsing. Currently just removes indentation.
pub fn sanitize(content: &str) -> String {
    let strip_indent = Regex::new(r#"\n\s+"#).unwrap();
    strip_indent.replace_all(content, "\n").to_string()
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LlmlParser;

impl LlmlParser {
    pub fn parse_string(content: &str) -> Result<Pair<Rule>, String> {
        let mut pairs = match Self::parse(Rule::File, content) {
            Ok(p) => p,
            Err(e) => return Err(format!("Syntax error while parsing input\n\n{}", e)),
        };

        match pairs.next() {
            Some(r) => Ok(r),
            None => Err("Failed to parse root rule".to_string()),
        }
    }
}
