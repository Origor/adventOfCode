use std::{fs, borrow::BorrowMut};

fn parse_line_to_int(line: &str) -> i32 {
    let mut result: Vec<char> = Vec::new();
    let mut is_break = false;
    let mut word = String::new();
    let mut word_vec: Vec<String> = Vec::new();
    let char_indices = line.char_indices();

    for (iterator, character) in char_indices.clone() {
        if character.is_digit(10) {
            add_char_to_vec(&character, result.borrow_mut());
            if iterator != 0 {
                is_break = true;
            }
        }
        else {
            word.push(character);
        }
        if is_break || iterator == line.char_indices().count() - 1 {
            word_vec.push(word.clone());
            word = String::new();
            is_break = false;
        }
    }
    return append_chars_to_one_int(&result);
}

fn add_char_to_vec(n: &char, vec: &mut Vec<char>) {
    vec.push(*n);
}

fn append_chars_to_one_int(vec: &Vec<char>) -> i32 {
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
    let mut result: Vec<i32> = Vec::new();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    for line in contents.lines() {
        result.push(parse_line_to_int(line));
    }
    println!("Answer: {}", sum_vec(&result));
}
