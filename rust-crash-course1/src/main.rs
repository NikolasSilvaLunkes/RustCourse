#![deny(clippy::all)]

//Consts
const MY_AGE: u8 = 19;

fn main() {
    examples();
}

fn examples() {
    //Immutable variables
    let first_name: &str = "Nikolas";
    let last_name: &str = "Lunkes";

    //Mutable Variables
    let mut age: u8 = 19;
    println!("Hello, {} {}!", first_name, last_name);
    println!("You are {} years old", age);
    age += 1u8;
    println!("Oh, now you are {} years old", age);
    println!("If time did not pass you would be {} years old", MY_AGE);
    //Exanoke if ints with decimals separation
    let population: i32 = 62_000_000;

    //Color
    let red: i32 = 0xFA;
    let rgb: i32 = 0xFF0000;

    //Floats
    let example_distance: f32 = 2.3;
    let distance1: f64 = 5.5;
    let distance2: f64 = 6.2;
    let distance_in_km: f64 = distance1 + distance2;
    println!("{distance_in_km}");

    //Shadowing
    let name: &str = "Nikolas";
    let name: &str = "Nikolas Silva Lunkes";
    let name: i32 = 123512;

    //Tuples
    let personal_data: (i32, &str) = (19i32, "Nikolas");
    let (age, name) = personal_data;
    let age: i32 = personal_data.0;
    let name = personal_data.1;
}
