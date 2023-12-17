use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let digit_strings: Vec<String> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(String::from)
    .to_vec();

    let rev_digit_strings: Vec<String> = digit_strings
        .iter()
        .map(|s| reverse_string(s))
        .collect::<Vec<String>>();

    let digit_first_chars: Vec<char> = digit_strings
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let digit_last_chars: Vec<char> = rev_digit_strings
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut sum = 0;

    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();

        let mut first_digit: u32 = 0;
        for (i, c) in l.chars().enumerate() {
            if c.is_digit(10) {
                first_digit = c.to_digit(10).unwrap();
                break;
            }

            if is_first_char_of_digit(&digit_first_chars, c) {
                let digit_string = is_digit_string(i, c, &l, &digit_strings);
                if digit_string == 0 {
                    continue;
                } else {
                    first_digit = digit_string;
                    break;
                }
            }
        }

        let mut second_digit: u32 = 0;
        for (i, c) in l.chars().rev().enumerate() {
            if c.is_digit(10) {
                second_digit = c.to_digit(10).unwrap();
                break;
            }

            if is_first_char_of_digit(&digit_last_chars, c) {
                let digit_string = is_digit_string(i, c, &reverse_string(&l), &rev_digit_strings);
                if digit_string == 0 {
                    continue;
                } else {
                    second_digit = digit_string;
                    break;
                }
            }
        }

        let concat_digits = (first_digit * 10) + second_digit;

        sum += concat_digits;
    }

    println!("Sum of numbers: {}", sum);
}

fn is_first_char_of_digit(digit_first_chars: &Vec<char>, c: char) -> bool {
    return digit_first_chars.contains(&c);
}

fn is_digit_string(c_index: usize, c: char, line: &str, digit_strings: &Vec<String>) -> u32 {
    for (idx, digit_string) in digit_strings.iter().enumerate() {
        // only check digit_strings that start/end with c
        let first_char = digit_string.chars().next().unwrap();
        if c != first_char {
            continue;
        }

        // test for each digit_string
        let substring = &line[c_index..];
        if substring.starts_with(digit_string) {
            return (idx + 1) as u32;
        }
    }
    return 0;
}

fn reverse_string(s: &String) -> String {
    let mut result = String::new();
    for c in s.chars().rev() {
        result += &c.to_string();
    }
    return result;
}
