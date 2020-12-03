use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct ArborealMap {
  tile_width: usize,
  map: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum MapLocation {
  Tree,
  Open
}

impl ArborealMap {
  pub fn new() -> ArborealMap {
    ArborealMap {
      tile_width: 0,
      map: Vec::new()
    }
  }

  pub fn count_for_slope(&mut self, delta_x: usize, delta_y: usize, find: MapLocation) -> usize {
    let locations = self.slope_locations(delta_x, delta_y);
    let mut count: usize = 0;

    for location in locations {
      if self.at(location.0, location.1) == find {
        count +=1;
      }
    }

    count
  }

  pub fn slope_locations(&mut self, delta_x: usize, delta_y: usize) -> Vec<(usize, usize)> {
    let mut locations = Vec::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    
    loop {
      locations.push((x, y));

      x += delta_x;
      y += delta_y;

      if y >= self.map.len() { break; }
    }

    locations
  }

  pub fn at(&mut self, x: usize, y: usize) -> MapLocation {
    let x_wrapped = x % self.tile_width;
    let row = self.map.get(y).unwrap();
    let c = row.chars().nth(x_wrapped).unwrap();
    match c {
      '#' => MapLocation::Tree,
      _ => MapLocation::Open
    }
  }

  pub fn add_row(&mut self, row: String) {
    self.tile_width = cmp::max(self.tile_width, row.len());
    self.map.push(row);
  }
}

pub fn read_map() -> Result<ArborealMap, std::io::Error> {
  File::open("input.txt")
    .map(|x| BufReader::new(x))
    .map(|r| {      
      let mut trees = ArborealMap::new();

      for line in r.lines() {
        match line {
            Ok(v) => trees.add_row(v),
            Err(_) => ()
        }
      }

      trees      
    })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_at_with_wrapping() {
    let mut map = ArborealMap::new();
    map.add_row(".##.#.".to_string());

    assert_eq!(map.at(0, 0), MapLocation::Open);
    assert_eq!(map.at(1, 0), MapLocation::Tree);
    assert_eq!(map.at(2, 0), MapLocation::Tree);
    assert_eq!(map.at(3, 0), MapLocation::Open);
    assert_eq!(map.at(4, 0), MapLocation::Tree);
    assert_eq!(map.at(5, 0), MapLocation::Open);

    // wrapped
    assert_eq!(map.at(6, 0), MapLocation::Open);
    assert_eq!(map.at(7, 0), MapLocation::Tree);
    assert_eq!(map.at(8, 0), MapLocation::Tree);
    assert_eq!(map.at(9, 0), MapLocation::Open);
    assert_eq!(map.at(10, 0), MapLocation::Tree);
    assert_eq!(map.at(11, 0), MapLocation::Open);
  }

  #[test]
  fn test_2d() {
    let mut map = ArborealMap::new();
    map.add_row("..".to_string());
    map.add_row(".#".to_string());
    map.add_row("#.".to_string());
    map.add_row("##".to_string());

    assert_eq!(map.at(0, 0), MapLocation::Open);
    assert_eq!(map.at(1, 3), MapLocation::Tree);
  }

  #[test]
  fn slope_location_test() {
    let mut map = ArborealMap::new();
    map.add_row("..".to_string());
    map.add_row(".#".to_string());
    map.add_row("#.".to_string());
    map.add_row("##".to_string());

    assert_eq!(map.slope_locations(1,1), vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    assert_eq!(map.slope_locations(3,1), vec![(0, 0), (3, 1), (6, 2), (9, 3)]);
  }

  #[test]
  fn slope_tree_count_test() {
    let mut map = ArborealMap::new();
    map.add_row("..".to_string());
    map.add_row(".#".to_string());
    map.add_row("#.".to_string());
    map.add_row("##".to_string());
    assert_eq!(map.count_for_slope(1, 1, MapLocation::Tree), 3);

    /*
    O.............
    .#.X.#.#.#.#.#
    #.#.#.X.#.#.#.
    #########X#### 
    */
    assert_eq!(map.count_for_slope(3, 1, MapLocation::Tree), 3);
  }
}
