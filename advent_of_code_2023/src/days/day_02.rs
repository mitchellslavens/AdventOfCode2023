use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug)]
struct Cubes {
    blue: u32,
    green: u32,
    red: u32,
}

type Game = Vec<Cubes>;

pub fn day_02_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input2.txt") {
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
        let max_colors = max_cubes(&line);
        if max_colors.blue <= 14 && max_colors.green <= 13 && max_colors.red <= 12 {
            sum += line.split(' ').nth(1).unwrap().replace(':', "").parse::<u32>().unwrap();
        }
    }
    println!("\tThe total sum for part a is {}", sum);
    Ok(())
}

pub fn day_02_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input2.txt") {
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
        let max_colors = max_cubes(&line);
        sum += max_colors.red * max_colors.green * max_colors.blue;
    }
    println!("\tThe total sum for part b is {}", sum);
    Ok(())
}


// Leaving this here since this is initially how I did part 1, but then opted to use "max_cubes" for both part 1 and part 2.
fn _is_valid_game(game: &str) -> bool {
    let round_cubes: Game = game.split(": ").nth(1).unwrap().split("; ").map(count_cubes).collect();
    
    let res = round_cubes.iter().filter(|c| c.blue > 14 || c.green > 13 || c.red > 12);
    if res.count() > 0 {
        return false
    } else {
        return true
    }
}

fn max_cubes(game: &str) -> Cubes {
    let round_cubes: Game = game.split(": ").nth(1).unwrap().split("; ").map(count_cubes).collect();
    let red = round_cubes.iter().max_by_key(|c| c.red).unwrap().red;
    let green = round_cubes.iter().max_by_key(|c| c.green).unwrap().green;
    let blue = round_cubes.iter().max_by_key(|c| c.blue).unwrap().blue;

    Cubes{ blue, green, red }
}

fn count_cubes(round: &str) -> Cubes {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;

    for cube in round.split(", ") {
        let mut splits = cube.split(' ');
        let count = splits.next().unwrap().parse::<u32>().unwrap();
        let color = splits.next().unwrap();

        match color {
            "blue" => blue += count,
            "green" => green += count,
            "red" => red += count,
            _ => panic!("Color {} is not recognized", color)
        }
    }
    Cubes { blue, green, red}
}