pub mod store {
    use std::collections::HashMap;
    
    pub fn update_old_value() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        
        println!("{:?}", map)
    }

    pub fn update() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Yellow")).or_insert(50);

        println!("{:?}", scores);
    }

    pub fn overwrite() {
        let mut scores = HashMap::new(); //
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 10);

        println!("{:?}", scores);
    }

    pub fn map() {
        let field_name = String::from("Favourite color");
        let field_value = String::from("Yellow");

        let mut map = HashMap::new();
        map.insert(field_name.clone(), field_value);
        let first = map.get(&field_name).cloned().unwrap_or_default();
        println!("map for {}: {}", field_name, first);

    }

    pub fn get_value() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Green"), 50);

        let team_name = String::from("Blue");
        let _score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }
}