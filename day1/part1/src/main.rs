use std::fs::File;
use std::io::{self, prelude::*, BufReader};
mod report;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut r = report::Report::new();

    for line in reader.lines() {
        let value: i32 = match line {
            Ok(line_string) => line_string.parse().expect("must be an integer"),
            Err(_) => 0,
        };

        r.add(value);
    }

    let prod_sum = r.prod_sum_2020();
    match prod_sum {
        Ok(v) => println!("{}", v),
        Err(()) => println!("No match :("),
    }
    

    Ok(())
}
