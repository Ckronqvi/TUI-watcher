use std::env;
use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::Path;

pub fn get_price_file_path() -> String {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/tui-watcher-last-price.txt", home_dir)
}

pub fn save_last_price(price: &str) -> Result<(), Box<dyn Error>> {
    let price_file_path = get_price_file_path();
    write(price_file_path, price)?;
    Ok(())
}

pub fn read_last_price() -> Option<String> {
    let price_file_path = get_price_file_path();
    if Path::new(&price_file_path).exists() {
        let price = read_to_string(price_file_path).unwrap_or_default();
        Some(price)
    } else {
        None
    }
}
