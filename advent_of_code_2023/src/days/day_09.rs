use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_09_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input9.txt") {
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
    let mut total = 0;
    for line in reader.lines() {
        let new_line = line?;
        let mut nums : Vec<i32> = new_line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        total += extrapolate(nums);
    }
    println!("\tThe total sum of values for part a is {}", total);
    Ok(())
}

fn extrapolate(nums: Vec<i32>) -> i32 {
    if nums.iter().filter(|&&x| x != 0).count() == 0 {
        return 0;
    }
    let mut new_nums: Vec<i32> = Vec::new();
    for i in 1..nums.len() {
        new_nums.push(nums[i] - nums[i-1]);
    }
    return nums.last().unwrap() + extrapolate(new_nums);
}


pub fn day_09_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input9.txt") {
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
    let mut total = 0;
    for line in reader.lines() {
        let new_line = line?;
        let mut nums : Vec<i32> = new_line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        total += extrapolate_backwards(nums);
    }
    println!("\tThe total sum of values for part b is {}", total);
    Ok(())
}

fn extrapolate_backwards(nums: Vec<i32>) -> i32 {
    if nums.iter().filter(|&&x| x != 0).count() == 0 {
        return 0;
    }
    let mut new_nums: Vec<i32> = Vec::new();
    for i in 1..nums.len() {
        new_nums.push(nums[i] - nums[i-1]);
    }
    return nums.first().unwrap() - extrapolate_backwards(new_nums);
}
