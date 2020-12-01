use std::collections::HashSet;

pub struct Report {
  entries: HashSet<i32>,
}

impl Report {
  pub fn new() -> Report {
    Report {
      entries: HashSet::new(),
    }
  }

  pub fn add(&mut self, value: i32) {
    self.entries.insert(value);
  }

  pub fn prod_sum3_2020(&mut self) -> Result<i32, ()> {
    for x in self.entries.iter() {
      for y in self.entries.iter() {
        let diff = 2020 - x - y;
        if self.entries.contains(&diff) {        
          return Ok(diff * x * y)
        }
      }
    }
    Err(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_test() {
    let mut r = Report::new();
    r.add(1721);
    r.add(979);
    r.add(366);
    r.add(299);
    r.add(675);
    r.add(1456);

    let prod_sum = r.prod_sum3_2020();

    assert!(prod_sum.is_ok(), "ok");
    assert_eq!(prod_sum.unwrap(), 241861950);
  }
}
