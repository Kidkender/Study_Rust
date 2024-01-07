use restaurant::customer;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);

    customer::eat_at_restaurant();
}