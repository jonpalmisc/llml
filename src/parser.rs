use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LlmlParser;

impl LlmlParser {
    pub fn parse_file_content(content: &str) -> Result<Pair<Rule>, String> {
        let mut pairs = match Self::parse(Rule::File, &content) {
            Ok(p) => p,
            Err(e) => return Err(format!("Syntax error while parsing input\n\n{}", e)),
        };

        match pairs.next() {
            Some(r) => return Ok(r),
            None => return Err(format!("Failed to retrieve root rule")),
        };
    }
}
