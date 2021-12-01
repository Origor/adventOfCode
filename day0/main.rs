use std::env;
use std::fs;

fn main() {
    let filename = "example_input.txt";
    println!("Hello, world!");
    println!("In file {}", filename);

    let contents: i32 = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    
    for text in contents {
        text.parse::<i32>().unwrap();
    }


    contents.collect::<Vec<i32>>();

    println!("With text:\n{}", contents);
}
