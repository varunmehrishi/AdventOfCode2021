use crate::utils::read_lines;
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

pub fn read_hexstring(file_path: &Path) -> Result<String, Error> {
    let mut lines = read_lines(file_path)?;

    if let Some(line) = lines.next() {
        Ok(line?)
    } else {
        Err(Error::from(ErrorKind::NotFound))
    }
}
