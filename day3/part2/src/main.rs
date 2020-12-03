mod arborealmap;

fn main() {
    let mut tree_map = arborealmap::read_map().unwrap();

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let product: usize = slopes.iter()
        .map(|slope| tree_map.count_for_slope(slope.0, slope.1, arborealmap::MapLocation::Tree))
        .product();
        
    println!("{}", product);
}
