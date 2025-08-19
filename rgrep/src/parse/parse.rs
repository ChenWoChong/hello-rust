use crate::error::RgrepError;
use crate::parse::{Args, Parse};
use std::io::BufRead;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parse for Parser {
    fn parse(&self) -> Result<Args, RgrepError> {
        todo!()
    }
}
