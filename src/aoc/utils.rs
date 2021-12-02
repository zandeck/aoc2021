use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn read_data<T>(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|v| match v {
        Ok(s) => T::from_str(&s).unwrap(),
        Err(_) => panic!("Error reading lines from file"),
    }))
}
