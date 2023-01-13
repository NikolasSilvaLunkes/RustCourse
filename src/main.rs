#![deny(clippy::all)]

fn say_hello_world() {
    let message: String = String::from("Hello, World");
    println!("{}", message);
}

fn get_hello_world() -> String {
    let message: String = String::from("Hello, World");
    message
}

fn say_hello_to_person(to_person: &str) {
    let message: String = String::from("Hello");
    println!("{}, {}", message, to_person);
}

fn say_hello_world_to_person(to_person: String) -> String {
    let message: String = format!("Hello, world {}!", to_person);
    message
}

fn main() {
    say_hello_world();
    println!("{}", get_hello_world());
    say_hello_to_person("Nikolas");
    println!("{}", say_hello_world_to_person(String::from("Nikolas")));

    let multiply_by_2: |i32| -> i32 |x: i32| x * 2;
        
}
