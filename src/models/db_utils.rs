use postgres::{rows::Row, types::FromSql};
use std::error::Error;

pub(super) fn get_cell<T>(row: &Row, column: &str) -> Result<T, Box<dyn Error>>
where
    T: FromSql,
{
    Ok(row.get_opt(column).ok_or("postgres: column not found")??)
}
