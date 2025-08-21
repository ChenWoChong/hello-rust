mod parse;

use crate::error::RgrepError;

pub struct Args<'a> {
    #[allow(dead_code)]
    key: &'a str,
    #[allow(dead_code)]
    file_name: &'a str,
}

pub trait Parse {
    #[allow(dead_code)]
    fn parse(&self) -> Result<Args, RgrepError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_should_work() {}
}
