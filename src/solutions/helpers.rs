use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub fn read_ints(path: &Path) -> Result<Vec<i64>, Error> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(
            line?
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}

pub fn read_lines(path: &Path) -> Result<Vec<String>, Error> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut v: Vec<String> = vec![];
    for line in br.lines() {
        v.push(line?);
    }
    Ok(v)
}