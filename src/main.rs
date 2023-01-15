#![deny(clippy::all)]

//hashmaps
use std::collections::HashMap;

fn ex1() {
    //Collections
    let (hello, world, number) = ("Hello".to_string(), "World".to_string(), 30);
}

fn ex2() {
    let values: [&str; 2] = ["foo", "bar"];
    let fooe: &&str = &values[0];
    for value in values.iter() {
        println!("Foo is {}", fooe);
    }
    let length: usize = values.len();
    println!("Length is {}", length)
}

fn ex3() {
    let values: [i32; 2] = [10, 20];
    //let doubled: impl Iterator<Item = i32> = values.iter().map(|x: &i32| x * 2);
}

fn ex4() {
    let mut values: Vec<i32> = vec![1, 2, 3];
    values.push(4);
    let four: Option<i32> = values.pop();
}

fn ex5() {
    //vector
    let mut values1: Vec<i32> = vec![1, 2, 3];
    let mut values2: Vec<i32> = vec![4, 5, 6];
    println!("values1 = {:?}", values1);
    println!("values2= {:?}", values2);
    //vector gets the other vector
    values1.append(&mut values2);
    println!("values1 = {:?}", values1);
    println!("values2= {:?}", values2);
    //check if one constains another
    if values1.contains(&3) {
        println!("yes");
    } else {
        println!("no");
    }
    if values2.is_empty() {
        println!("empty");
    } else {
        println!("not empty");
    }

    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert(k: &"foo", v: "bar");
    if values.contains("name"){
        println!("name exists");
    } else {
        println!("no name");
    }
}

fn main() {
    ex1();
    ex2();
    ex4();
    ex5();
}
