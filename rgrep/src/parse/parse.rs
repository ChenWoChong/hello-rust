use crate::error::RgrepError;
use crate::parse::{Args, Parse};

pub struct Parser {}

impl Parser {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Parse for Parser {
    fn parse(&self) -> Result<Args, RgrepError> {
        todo!()
    }
}
