use vec_funhouse::*;
fn main() {
    let config = RangeConfig { 
        low: 4,
        high: 6,
        include_high: true,
        include_low: true
    };

    let mut my_vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{my_vector:?}");
    filter_range(&mut my_vector, &config, RangeMode::KeepOutside);
    println!("{my_vector:?}");
}