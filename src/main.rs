mod parser;
extern crate pest_derive;
use quote::*;
use syn::{export::TokenStream2};



fn main() {
    let gend = parser::parse();
    dbg!(gend);
}
