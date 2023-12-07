use std::fs::File;
use std::io::{BufReader, BufRead};

// Math stuff:
// t is travel time
// r is race time
// b is button pressed time
// d is distance traveled
// t = r - b
// d = t * b
// So...
// d = (r - b) * b 
// d = r * b - b^2
// b^2 - r * b + d = 0
// So, quadratic formula with a = 1, b = -r, and c = d

// This is the brute force way, but it could have been sped up with a binary search.
pub fn day_06_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input6.txt") {
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
    let mut reader = BufReader::new(file);
    let mut new_line = String::new();
    let mut total_ways_score = 1;

    reader.read_line(&mut new_line)?;

    let race_times: Vec<u32> = 
            new_line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
    
    new_line.clear();
    reader.read_line(&mut new_line)?;
    let race_records: Vec<u32> = 
            new_line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

    // Go through each race
    for i in 0..race_times.len() {
        let mut ways = 0;
        for x in 1..race_times[i] {
            if (race_times[i] * x) - x.pow(2) > race_records[i] {
                ways += 1;
            }
        }
        if ways > 0 {
            total_ways_score *= ways;
        }
    }


    println!("\tThe total sum for part b is {}", total_ways_score);
    Ok(())
}


// Using math way (solve quadratic formula to find min/max x)
pub fn day_06_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input6.txt") {
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
    let mut reader = BufReader::new(file);
    let mut new_line = String::new();

    reader.read_line(&mut new_line)?;

    let r = new_line.split(":").nth(1)
    .unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap();

    new_line.clear();
    reader.read_line(&mut new_line)?;
    let d = new_line.split(":").nth(1)
    .unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap();

    // Quadratic formula with a = 1, b = -r, and c = d

    let end = ((r + (r.powf(2.0) - 4.0 * d).sqrt()) / 2.0).floor();
    let start = ((r - (r.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil();

    println!("\tThe total winning ways for part b is {}", (end - start + 1.0));
    Ok(())
}
