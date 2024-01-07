pub fn store() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    // let first = &v[0];
    
    println!("The third element is {third}"); 

    let third: Option<&i32> = v.get(100);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There iss no third element."),
    }
}