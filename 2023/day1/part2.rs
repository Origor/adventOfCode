use regex::Regex;
use core::panic;
use std::{borrow::BorrowMut, fs};

fn add_char_to_vec(c: &char, vec: &mut Vec<char>) {
    vec.push(*c);
}

fn append_first_and_last_char_to_one_digit(vec: &Vec<char>) -> i32 {
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

fn match_string_to_single_digit(to_parse: &str) -> Option<char> {
    match to_parse {
        "one"   => return Some('1'),
        "two"   => return Some('2'),
        "three" => return Some('3'),
        "four"  => return Some('4'),
        "five"  => return Some('5'),
        "six"   => return Some('6'),
        "seven" => return Some('7'),
        "eight" => return Some('8'),
        "nine"  => return Some('9'),
        _ => None,
    }
} 

fn split_string_to_digits(to_parse: &str) -> Option<Vec<char>> {
    let mut char_vec: Vec<char> = Vec::new();
    let mut skip_iter: Option<usize> = None;
    let regexp = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for cap in to_parse.char_indices().filter_map(|(i,_)| regexp.find(&to_parse[i..])) {
        if skip_iter.is_some() && skip_iter.unwrap() == 0 {
            skip_iter = None;
        } else if skip_iter.is_some() && skip_iter.unwrap() > 0 {  
            skip_iter = Some(skip_iter.unwrap()-1);
            continue;
        }
        if !cap.is_empty() {
            skip_iter = Some(cap.end()-2);
            let digit_match = match_string_to_single_digit(cap.as_str());
            if digit_match.is_some() {
                char_vec.push(digit_match.unwrap());
            }
        }
    }

    if char_vec.is_empty() {
        return None;
    }
    return Some(char_vec);
}

fn parse_string_slice_and_push_to_vector(to_parse: &String, vec: &mut Vec<char>) {
    let slice_digit_vec = split_string_to_digits(to_parse.as_str());
    if let Some(slice_digit_vec) = slice_digit_vec {
        for item in slice_digit_vec {
            add_char_to_vec(&item, vec);
        }
    }
}

fn line_to_first_and_last_i32(line: &str, part1: &bool) -> i32 {
    let mut nums_in_line: Vec<char> = Vec::new();
    let mut string_slice = String::new();
    let mut digit_char: Option<char> = None;
    let mut has_string_slice = false;

    // iterate over the line and add parsed numbers to the vector
    for (iterator, character) in line.char_indices().clone() {
        // if the character is a digit, mark the char to be inserted
        if character.is_digit(10) { // 10 is the base of the number system
            digit_char = Some(character);
            if !string_slice.is_empty() {
                has_string_slice = true;
            }
            if *part1 || iterator == 0 || iterator > 0 && string_slice.is_empty() {
                add_char_to_vec(&digit_char.unwrap(), nums_in_line.borrow_mut());
                continue;
            }
        } else {
            string_slice.push(character);
        }
        if has_string_slice && digit_char.is_some() {
            parse_string_slice_and_push_to_vector(&string_slice, nums_in_line.borrow_mut());
            add_char_to_vec(&digit_char.unwrap(), nums_in_line.borrow_mut());

            // reset the variables since we inserted a new number
            string_slice = String::new();
            digit_char = None;
            has_string_slice = false;
            continue;
        }
        // if there are no digits in the line, add the string slice to the vector
        if iterator == line.char_indices().count() - 1 {
            parse_string_slice_and_push_to_vector(&string_slice, nums_in_line.borrow_mut());
            continue;
        }
    }

    if nums_in_line.is_empty() {
        panic!("No numbers in line");
    }
    return append_first_and_last_char_to_one_digit(&nums_in_line);
}

fn parse_file_to_i32(input: &String, part1: &bool) -> i32 {
    let mut result: Vec<i32> = Vec::new();

    for line in input.lines() {
        result.push(line_to_first_and_last_i32(line, part1));
    }

    return sum_vec(&result);
}

fn main() {
    let input1_example = fs::read_to_string("input1Example.txt")
        .expect("Should have been able to read the file");
    let input2_example = fs::read_to_string("input2Example.txt")
        .expect("Should have been able to read the file");
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let part1 = true;

    println!("Part1: {}", parse_file_to_i32(&input1_example, &part1));
    println!("Part1 input: {}", parse_file_to_i32(&input, &part1));
    println!("Part2: {}", parse_file_to_i32(&input2_example, &!part1));
    println!("Part2 input: {}", parse_file_to_i32(&input, &!part1));
}
