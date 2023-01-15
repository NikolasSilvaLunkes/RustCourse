#![deny(clippy::all)]

use std::fmt::format;



fn get_first_name() -> Result<String, ()> {
    Err(())
}


fn get_last_name() -> Result<String, ()> {
    Ok("Doe".to_string())
}

fn get_full_name() -> Result<String, ()> {
    let first_name: String = get_first_name()?;
    let last_name: String = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let full_name: Result<String, ()> = get_full_name();
    match full_name {
        Ok(name) => println!("Hello, {}!", name),
        Err(_) => println!("Error"),
    }
}
