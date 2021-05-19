extern crate pest;
use pest_derive::*;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"] // relative to src
struct MyParser;

fn parse_field(pair: Pair<Rule>, st: &mut Block) {
    let mut it = pair.into_inner();
    let name = it.next().unwrap().as_str();
    let size: u32 = it.next().unwrap().as_str().parse().unwrap();
    st.fields.push(Field::Val(name.to_string(), size));

}
fn parse_padding(pair: Pair<Rule>, st: &mut Block) {
    st.fields.push(Field::Padding(pair.into_inner().next().unwrap().as_str().parse().unwrap()));
}

fn parse_block_content(pair: Pair<Rule>, st: &mut Block) {
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::field => parse_field(p, st),
            Rule::padding => parse_padding(p, st),
            all => eprintln!("in parse block content: {:?}", all),
        }
    }
}

fn parse_block(pair: Pair<Rule>) -> Block {
    let mut block = Block {
        name: String::new(),
        fields: vec![],
    };
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident => {block.name = p.as_str().trim_start_matches("seL4_").to_string();}
            Rule::block_content => parse_block_content(p, &mut block),
            all => eprintln!("in parse block: {:?}", all),
        }
    }
    block
}

#[derive(Debug, Clone)]
pub enum Field {
    Padding(u32),
    Val(String, u32),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
    pub fields: Vec<Field>
}

pub fn parse() -> Vec<Block> {
    let string = include_str!("../../shared_types.bf").to_string();
    // the grammar.pest can't handle the new line at end of file
    let string = string.trim_end_matches("\n");

    let pairs = MyParser::parse(Rule::parser, string).unwrap();
    let mut vecs = vec![];

    for pair in pairs {
        match pair.as_rule() {
            // Currently assumes base = 64
            Rule::base => {},
            Rule::block => {
                vecs.push(parse_block(pair));
            }

            // TODO: handle ifdefs and includes.
            _ => eprintln!("top lvl parse: {:#?}", pair),
        }

    }
    vecs
}
