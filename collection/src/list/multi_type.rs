#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn examp(number: i32, text: &str, decimal: f64) {
    let _row = vec![
        SpreadsheetCell::Int(number),
        SpreadsheetCell::Text(String::from(text)),
        SpreadsheetCell::Float(decimal),
    ];
    println!("{:?}", _row.get(0));
}