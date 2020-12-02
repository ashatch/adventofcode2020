use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
mod validator;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let mut record = validator::PasswordRecord::from_str(&line.unwrap()).unwrap();        
        if record.is_compliant() {
            count = count + 1;
        }
    }

    println!("{}", count);


    Ok(())
}
