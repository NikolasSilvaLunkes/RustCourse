#![deny(clippy::all)]

fn empty_string(value: &mut String) {
    value.clear();
}

fn get_name() -> &String {
    &"John".to_string();
}

fn main() {
    //Onwership
    //The name1 value refers to String::from("Nikolas");
    let name1: String = String::from("Nikolas");
    //The name2 value steals the String::from("Nikolas");
    let name2: String = name1;
    //println!("{name1}"); <-- Actual code
    println!("{}", name2);

    let i1: i32 = 10;
    let i2: i32 = i1;
    println!("You are {} year old", i1);
    println!("You are {} year old", i2);

    //Onwership
    //The name1 value steals String::from("Nikolas");
    let name1: String = String::from("Nikolas");
    //The name2 value refers name1;
    let name2: &String = &name1;

    println!("{name1}");
    println!("{}", name2);

    let mut ex: String = String::from("Nikoals");
    let ex2: &mut String = &mut ex;
    empty_string(ex2);
    println!("{}", ex);
    println!("{}", ex2);

    println!("{}", get_name())
}
