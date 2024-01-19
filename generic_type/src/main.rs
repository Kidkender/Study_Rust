mod utils;
mod types;
use crate::utils::highest;
use crate::types::data_type;
use crate::types::distance;
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = highest::find_largest_number(&number_list);
    println!("The largest number is {}", result);

   let p = data_type::Point::new(5.0, 100.0); 
    println!("p.x = {}", p.x());

    let calculate = p.distance_from_origin(); 
    println!("Distance from origin: {}", calculate);
}
