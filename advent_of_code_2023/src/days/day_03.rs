use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


pub fn day_03_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input3.txt") {
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
    let mut parts: HashMap<(usize, usize), u32> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut gears: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
    let mut part_nums_sum = 0;
    populate_maps(reader, &mut parts, &mut symbols, &mut gears)?;
    // Now we have our maps of parts and symbols.
    for part in parts.iter() {
        let valid_adjacents = get_adjacents(part.0, part.1.to_string().len());
        for adj in valid_adjacents {
            if symbols.contains_key(&adj) {
                part_nums_sum += part.1;
            }
        }
    }

    println!("\tThe total parts number sum is {}", part_nums_sum);
    Ok(())
}


pub fn day_03_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input3.txt") {
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
    let mut parts: HashMap<(usize, usize), u32> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut gears: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
    
    populate_maps(reader, &mut parts, &mut symbols, &mut gears)?;
    // Now we have our maps of parts and symbols and gears
    for part in parts.iter() {
        let valid_adjacents = get_adjacents(part.0, part.1.to_string().len());
        for adj in valid_adjacents {
            if gears.contains_key(&adj) {
                // If we find an adjacent gear, increment the number of parts adjacent to it and calculate the new gear ratio.
                gears.insert(adj, (gears[&adj].0 + 1, gears[&adj].1 * part.1));
            }
        }
    }

    let mut gear_ratio_sum = 0;
    for gear in gears.keys() {
        if gears[gear].0 == 2 {
            gear_ratio_sum += gears[gear].1;
        }
    }
    println!("\tThe total gear ratio sum is {}", gear_ratio_sum);

    Ok(())
}

fn populate_maps(reader: BufReader<File>, parts: &mut HashMap<(usize, usize), u32>, symbols: &mut HashMap<(usize, usize), char>, gears: &mut HashMap<(usize, usize), (u32, u32)>) -> std::io::Result<()> {
    let mut y_coord = 0;
    for line in reader.lines() {
        // First, we will create a map of all "parts" and another with all symbols. 
        // For parts, Key: (x,y) & Value: number
        // For symbols Key: (x,y) & Value : char
        let line = line?;
        let mut x_coord: usize = 0;
        let mut curr_num: String = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                // In this case, we want to keep adding to our number. 
                curr_num.push(c);
            }
            else {
                match curr_num.parse::<u32>() {
                    Ok(num) => {   
                        parts.insert((x_coord - curr_num.len(), y_coord), num);
                    },
                    Err(_) => { }
                }
                if c != '.' {
                    // This means there's a symbol.
                    // For part 2, we need to check if it is a gear and if it is we should populate our gears map.
                    if c == '*' {
                        gears.insert((x_coord, y_coord), (0, 1));
                    }
                    // Either way, we insert the symbol
                    symbols.insert((x_coord, y_coord), c);
                }
                curr_num.clear();
            }
            x_coord += 1;
        }
        match curr_num.parse::<u32>() {
            Ok(num) => {
                parts.insert((x_coord - curr_num.len(), y_coord), num);
                curr_num.clear();
            },
            Err(_) => {}
        }
        y_coord += 1;
    }

    Ok(())
}

fn get_adjacents(location: &(usize, usize), num_size: usize) -> Vec<(usize, usize)> {

    let mut adjacents: Vec<(usize, usize)> = Vec::new();
    let mut x_coord = location.0;
    let y_coord = location.1;

    // This is to make sure we don't go below 0
    if x_coord > 0 {
        x_coord -= 1;
    }
    
    for i in x_coord..(x_coord + num_size + 2) {
        if y_coord > 0 {
            adjacents.push((i, y_coord - 1));
        }
        adjacents.push((i, y_coord));
        adjacents.push((i, y_coord + 1));
    }
    adjacents
}