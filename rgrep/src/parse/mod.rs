mod parse;

use crate::error::RgrepError;
pub use parse::Parser;
use std::io;

pub struct Args<'a> {
    key: &'a str,
    file_name: &'a str,
}

pub trait Parse {
    fn parse(&self) -> Result<Args, RgrepError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_should_work() {}
}
