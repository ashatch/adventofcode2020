mod arborealmap;

fn main() {
    let count = arborealmap::read_map().map(|mut m| m.count_for_slope(3, 1, arborealmap::MapLocation::Tree));

    match count {
        Ok(value) => println!("{} trees", value),
        Err(_) => println!("Couldn't load the tree map")
    }
}
