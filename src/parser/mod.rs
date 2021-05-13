extern crate pest;
#[macro_use]
use pest_derive::*;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"] // relative to src
struct MyParser;

fn parse_field(pair: Pair<Rule>, st: &mut Generated) {
    let mut it = pair.into_inner();
    let name = it.next().unwrap().as_str();
    let size: u32 = it.next().unwrap().as_str().parse().unwrap();
    st.fields.push(Field::Val(name.to_string(), size));

}
fn parse_padding(pair: Pair<Rule>, st: &mut Generated) {
    st.fields.push(Field::Padding(pair.into_inner().next().unwrap().as_str().parse().unwrap()));
}

fn parse_block_content(pair: Pair<Rule>, st: &mut Generated) {
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::field => parse_field(p, st),
            Rule::padding => parse_padding(p, st),
            _ => {}
        }
    }
}

fn parse_block(pair: Pair<Rule>) -> Generated {
    let mut gen = Generated {
        name: String::new(),
        fields: vec![],
    };
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident => {gen.name = p.as_str().to_string();}
            Rule::block_content => parse_block_content(p, &mut gen),
            all => println!("in parse block: {:?}", all),
        }
    }
    gen
}

#[derive(Debug, Clone)]
pub enum Field {
    Padding(u32),
    Val(String, u32),
}
#[derive(Debug, Clone)]
pub struct Generated {
    pub name: String,
    pub fields: Vec<Field>
}

pub fn parse() -> Vec<Generated> {
    // let mut syscalls = vec![];
    let string = include_str!("../../shared_types.bf").to_string();
    let string = string.trim_end_matches("\n");
    // let string = include_str!("../testbf.bf").to_string();
    let pairs = MyParser::parse(Rule::parser, string).unwrap();
    let mut vecs = vec![];
    for pair in pairs {
        // A pair can be converted to an iterator of the tokens which make it up:
        match pair.as_rule() {
            Rule::base => {
                print!("base = ");
                println!("{}", pair.clone().into_inner().as_str());
            },

            Rule::block => {
                vecs.push(parse_block(pair));
            }
            _ => println!("{:#?}", pair),
        }

    }
    vecs
}
