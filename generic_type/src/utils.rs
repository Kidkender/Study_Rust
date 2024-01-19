
pub mod highest {
    pub fn find_largest_number(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
    
        for item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    
}




