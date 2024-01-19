use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new(); 
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
} 
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let username_result = read_username_from_file();

    match username_result {
        Ok(username) => {
            if let Some(last_char) = last_char_of_first_line(&username) {
                println!("Last char of the first line: {:?}", last_char);
            } else {
                println!("Text empty or has no lines");
            }
        }
        Err(e) => {
            println!("Error reading username: {:?}", e);
        }
    }
}
