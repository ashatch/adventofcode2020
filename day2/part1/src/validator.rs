use std::fmt;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
  pub min: i32,
  pub max: i32,
  pub seq: String,
}

#[derive(Debug, PartialEq)]
pub struct PasswordRecord {
  pub policy: PasswordPolicy,
  pub password: String,
}

impl PasswordRecord {
  pub fn is_compliant(&self) -> bool {
    let char_count = self.password.chars()
      .filter(|x| x.to_string() == self.policy.seq)
      .count() as i32;
    
    return char_count >= self.policy.min && char_count <= self.policy.max
  }
}

impl fmt::Display for PasswordPolicy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{} {}", self.min, self.max, self.seq)
  }
}

impl fmt::Display for PasswordRecord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.policy, self.password)
  }
}

impl FromStr for PasswordRecord {
  type Err = std::num::ParseIntError;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(?x)
      (?P<min>\d+)              # min number   
      -                         # 1x hyphen separator
      (?P<max>\d+)              # max number
      \s                        # 1x space separator
      (?P<seq>[A-Za-z0-9]{1})   # sequence string (1 char for now)
      :\s                       # 1x colon
      (?P<pw>[A-Za-z0-9]+)      # password
      ").unwrap();
    }

    let record = RE.captures(line).unwrap();
    let min = record["min"].parse::<i32>().unwrap();
    let max = record["max"].parse::<i32>().unwrap();
    let seq = record["seq"].to_string();
    let password = record["pw"].to_string();

    let policy = PasswordPolicy {
      min, max, seq
    };
    
    Ok(PasswordRecord { 
      policy, password
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_record_test() {
    let pr = PasswordRecord {
      policy: PasswordPolicy {
        min: 6,
        max: 23,
        seq: "x".to_string(),
      },
      password: "acdefg".to_string()
    };

    let parsed = PasswordRecord::from_str(&r"6-23 x: acdefg").unwrap();

    assert_eq!(parsed, pr);    
  }

  #[test]
  fn is_compliant_test() {
    let mut compliant_record = PasswordRecord::from_str(&r"1-3 a: abcde").unwrap();
    assert_eq!(compliant_record.is_compliant(), true);    
  }

  #[test]
  fn is_not_compliant_test() {
    let mut compliant_record = PasswordRecord::from_str(&r"1-3 b: cdefg").unwrap();
    assert_eq!(compliant_record.is_compliant(), false);    
  }
}
