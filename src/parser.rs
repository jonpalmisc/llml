use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LtmlParser;

impl LtmlParser {
    pub fn parse_file_content(content: &str) -> Pair<Rule> {
        Self::parse(Rule::File, &content).unwrap().next().unwrap()
    }
}
