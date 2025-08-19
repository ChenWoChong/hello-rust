use crate::parse::{Parse, Parser};

mod error;
mod parse;

fn main() {
    let parser = Parser::new();
    let res = parser.parse().unwrap();
    
    // read content
    
    // save to file
    
    //return
}
