use std::fs::File;
use std::io::{BufReader, BufRead};
use aho_corasick::AhoCorasick;

pub fn day_01_a() -> std::io::Result<()> {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok(());
                },
                _ => return Err(error),
            }
        }
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line= line?;
        sum += determine_number(line.as_bytes());
        
    }
    println!("\tThe total sum for part a is {}", sum);
    Ok(())
}

pub fn day_01_b() -> std::io::Result<()> {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok(());
                },
                _ => return Err(error),
            }
        }
    };
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        // I'd like to call replace_numbers here, but it isn't working for some reason.
        let new_line = replace_numbers2(&line?);
        sum += determine_number(new_line.as_bytes());
    }
    println!("\tThe total sum for part b is {}", sum);
    Ok(())
}

fn determine_number(s: &[u8]) -> u32 {
    let mut left_i = 0;
    let mut right_i = s.len() - 1;
    while left_i <= right_i && !(s[left_i].is_ascii_digit() && s[right_i].is_ascii_digit()) {
        if !s[left_i].is_ascii_digit() { left_i += 1; }
        if !s[right_i].is_ascii_digit() { right_i -= 1; }
    }
    let first_digit = s[left_i] as char;
    let second_digit = s[right_i] as char;
    
    let res = first_digit.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap();

    return res
}

// Somehow, this doesn't work. Not exactly sure why... It doesn't replace everything. It misses some.
fn _replace_numbers(s: &str) -> String {
    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replace_with = &["o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e"];
    let ac = AhoCorasick::builder()
        .build(patterns)
        .unwrap();
    ac.replace_all(s, replace_with)
}

fn replace_numbers2(s: &str) -> String {
    return s
    .replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e");
}