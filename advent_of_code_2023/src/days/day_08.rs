use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use num_integer::lcm;

pub fn day_08_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input8.txt") {
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
    let directions: Vec<_> = new_line.trim().chars().collect();
    let directions_len = directions.len();
    // Now read in the blank line
    reader.read_line(&mut new_line)?;

    // Now read in the map
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    new_line.clear();
    while reader.read_line(&mut new_line).is_ok() && !new_line.is_empty() {
        let key = new_line.split(" = ").nth(0).unwrap().trim();
        let l_value = 
            new_line.split(" = ")
                .nth(1).unwrap().split(",").nth(0).unwrap()
                .chars().skip(1).take(3).collect::<String>();
        let r_value = 
            new_line.split(" = ")
                .nth(1).unwrap().split(",").nth(1).unwrap()
                .chars().skip(1).take(3).collect::<String>();
        map.insert(key.to_string(), (l_value, r_value));
        new_line.clear();
    }
    
    // Here is how we can loop
    let mut i = 0;
    let mut curr = "AAA";

    while curr != "ZZZ" {
        if directions[i % directions_len] == 'L' {
            curr = &map[curr].0;
        } else {
            curr = &map[curr].1;
        }
        i += 1;
    }

    println!("\tThe total steps for part a is {}", i);
    Ok(())
}

pub fn day_08_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input8.txt") {
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
    let directions: Vec<_> = new_line.trim().chars().collect();
    let directions_len = directions.len();
    // Now read in the blank line
    reader.read_line(&mut new_line)?;

    // Now read in the map
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut curr_list: Vec<String> = Vec::new();
    new_line.clear();
    while reader.read_line(&mut new_line).is_ok() && !new_line.is_empty() {
        let key = new_line.split(" = ").nth(0).unwrap().trim();
        if key.ends_with('A') {
            curr_list.push(key.to_string());
        }
        let l_value = 
            new_line.split(" = ")
                .nth(1).unwrap().split(",").nth(0).unwrap()
                .chars().skip(1).take(3).collect::<String>();
        let r_value = 
            new_line.split(" = ")
                .nth(1).unwrap().split(",").nth(1).unwrap()
                .chars().skip(1).take(3).collect::<String>();
        map.insert(key.to_string(), (l_value, r_value));
        new_line.clear();
    }
    
    // Here is how we can loop
    let mut i = 0;
    let mut curr = "AAA";

    let mut first_z: Vec<usize> = vec![0; curr_list.len()];
    let mut first_z_found = 0;

    while first_z_found < curr_list.len() {
        let mut use_l = true;
        if directions[i % directions_len] == 'R' {
            use_l = false;
        } 
        for k in 0..curr_list.len() {
            if curr_list[k].ends_with('Z') {
                continue;
            }
            if use_l {
                curr_list[k] = map[&curr_list[k]].0.clone();
            }
            else {
                curr_list[k] = map[&curr_list[k]].1.clone();
            }
            if curr_list[k].ends_with('Z') {
                first_z[k] = i + 1;
                first_z_found += 1;
            }
        }
        i += 1;
    }

    let t_steps = first_z.iter().fold(1, |acc, x| lcm(acc, *x));

    println!("\tThe total steps for part b is {}", t_steps);
    Ok(())
}
