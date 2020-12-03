use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
mod validator;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let count = reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| validator::PasswordRecord::from_str(&line.unwrap()))
        .filter_map(|record| record.ok())
        .filter(|r| r.is_compliant())
        .count();

    println!("{}", count);


    Ok(())
}
