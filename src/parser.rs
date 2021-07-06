use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LlmlParser;

impl LlmlParser {
    pub fn parse_file_content(content: &str) -> Pair<Rule> {
        Self::parse(Rule::File, &content).unwrap().next().unwrap()
    }
}
