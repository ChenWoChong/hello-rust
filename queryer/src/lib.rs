use anyhow::{Result, anyhow};
use convert::Sql;
use fetcher::retrieve_data;
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::ops::{Deref, DerefMut};

mod convert;
mod dialect;
mod fetcher;
mod loader;

use crate::loader::detect_content;
pub use dialect::TyrDialect;
pub use dialect::example_sql;

#[derive(Debug)]
pub struct DataSet(DataFrame);

impl Deref for DataSet {
    type Target = DataFrame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by.into_iter().fold(filtered, |acc, (col, desc)| {
        acc.sort(
            &col,
            SortOptions {
                descending: desc,
                nulls_last: true,
                multithreaded: false,
                maintain_order: false,
            },
        )
    });

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX) as IdxSize);
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
