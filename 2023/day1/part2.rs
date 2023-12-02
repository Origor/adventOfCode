use std::{borrow::BorrowMut, fs};
use bimap::BiHashMap;

fn parse_string_to_i32(n: &String, vec: &mut Vec<char>) {
    println!("My string: {:?}", n);
    //vec.push(is_number_in_string(n.as_str()));
}

fn is_number_in_string(n: &str) -> Option<char> {
    let number_table: BiHashMap<&str, Option<i32>> = BiHashMap::<&str, Option<i32>>::from([
        ("", None),
        ("one", Some(1)),
        ("two", Some(2)),
        ("three", Some(3)),
        ("four", Some(4)),
        ("five", Some(5)),
        ("six", Some(6)),
        ("seven", Some(7)),
        ("eight", Some(8)),
        ("nine", Some(9)),
    ]);
    
    match number_table.get_by_left(left).(n) {
        Some(key) => return Some(format!("{:?}", key.unwrap() as u8 as char)),
        None => return None,
    }
}

fn parse_line_to_i32(line: &str) -> i32 {
    let mut result: Vec<char> = Vec::new();
    let mut is_break = false;
    let mut word = String::new();
    let char_indices = line.char_indices();

    for (iterator, character) in char_indices.clone() {
        if character.is_digit(10) {
            add_char_to_vec(&character, result.borrow_mut());
            if iterator != 0 {
                is_break = true;
            }
        } else {
            word.push(character);
        }
        if is_break || iterator == line.char_indices().count() - 1 {
            parse_string_to_i32(&word, result.borrow_mut());
            word = String::new();
            is_break = false;
        }
    }
    return append_chars_to_one_i32(&result);
}

fn parse_file_to_i32(contents: &String) -> i32 {
    let mut result: Vec<i32> = Vec::new();

    for line in contents.lines() {
        result.push(parse_line_to_i32(line));
    }

    return sum_vec(&result);
}

fn add_char_to_vec(n: &char, vec: &mut Vec<char>) {
    vec.push(*n);
}

fn append_chars_to_one_i32(vec: &Vec<char>) -> i32 {
    let mut my_string: String = String::new();
    my_string.push(*vec.first().unwrap());
    my_string.push(*vec.last().unwrap());
    return my_string.parse::<i32>().unwrap();
}

fn sum_vec(result: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in result {
        sum += i;
    }
    return sum;
}

fn main() {
    let contents =
        fs::read_to_string("input2Example.txt").expect("Should have been able to read the file");

    println!("Answer: {}", parse_file_to_i32(&contents));
}
