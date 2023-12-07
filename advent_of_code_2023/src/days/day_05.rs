use std::fs::File;
use std::io::{BufReader, BufRead, Read};

#[derive(Debug)]
struct CustomRange {
    source_start: i64,
    source_end: i64,
    modifier: i64,
}

impl CustomRange {
    fn contains(&self, value: i64) -> bool {
        value >= self.source_start && value < self.source_end
    }
}

pub fn day_05_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input5.txt") {
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
    let first_line = reader.by_ref().lines().next().unwrap()?;
    let seeds: Vec<i64> = first_line.split(":").nth(1).unwrap()
    .split(" ").filter_map(|x| x.parse::<i64>().ok()).collect();

    let mut ranges: Vec<CustomRange> = Vec::new();
    let mut buffer = String::new();

    // First, advance the buffer to the "seed-to-soil map"
    while !buffer.contains("map") {
        reader.read_line(&mut buffer)?;
    }
    buffer.clear();
    // Next, read in everything in that map up to the next map. Also, ignore empty lines.
    while !buffer.contains("map") {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        if buffer.len() < 3 || buffer.contains("map") {
            continue;
        }
        let line_nums: Vec<i64> = buffer.split(" ")
        .filter_map(|x| x.trim().parse::<i64>().ok()).collect();

        ranges
        .push(CustomRange { source_start: line_nums[1],
            source_end: line_nums[1] + line_nums[2],
            modifier: line_nums[0] - line_nums[1],
         });
    }

    let mut nums: Vec<i64> = Vec::new();

    for seed in seeds {
        let new_num = ranges.iter().find_map(|x| x.contains(seed).then(||seed + x.modifier));
        if let Some(new_num) = new_num{
            nums.push(new_num);
        } else {
            nums.push(seed);
        }
    }

    // This is very hacky, but run this 6 times (one for each map after the intial one)
    for _i in 0..6 {
        ranges.clear();
        buffer.clear();
        while !buffer.contains("map") {
            buffer.clear();
            reader.read_line(&mut buffer)?;
            if buffer.is_empty() {
                break;
            }
            if buffer.len() < 3 || buffer.contains("map") {
                continue;
            }
            let line_nums: Vec<i64> = buffer.split(" ")
            .filter_map(|x| x.trim().parse::<i64>().ok()).collect();

            ranges
            .push(CustomRange { source_start: line_nums[1],
                source_end: line_nums[1] + line_nums[2],
                modifier: line_nums[0] - line_nums[1],
            });
        }

        let mut nums2: Vec<i64> = Vec::new();

        for seed in nums {
            let new_num = ranges.iter().find_map(|x| x.contains(seed).then(||seed + x.modifier));
            if let Some(new_num) = new_num{
                nums2.push(new_num);
            } else {
                nums2.push(seed);
            }
        }

        nums = nums2;
    }
    

    println!("\tThe lowest location number for part a is {}", nums.iter().min().unwrap());

    Ok(())
}