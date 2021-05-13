mod parser;
extern crate pest_derive;


mod generator;

fn main() {
    let parsed = parser::parse();
    let generated = generator::gen(parsed);

    std::fs::write("generated.rs", generated.to_string()).unwrap();
}

