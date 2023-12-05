use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

pub fn day_04_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input4.txt") {
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
    let mut total_points = 0;
    for line in reader.lines() {
        let new_line = line?;
        let set: HashSet<&str> = HashSet::from_iter(new_line.split(":")
        .nth(1).unwrap().split("|").nth(0).unwrap().split(" ").filter(|&x| x != ""));
        let my_numbers = new_line.split("|")
        .nth(1).unwrap().split(" ").filter(|&x| x != "");
        let mut count = 0;
        //print!("Set = {:?} MyNumbers = ", set);
        for number in my_numbers {
            //print!("{}, ", number);
            if set.contains(number) {
                count += 1;
            }
        }
        if count > 0 {
            total_points += 1 << (count - 1);
        }
        
    }
    println!("\tThe total sum of points for part a is {}", total_points);
    Ok(())
}


pub fn day_04_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input4.txt") {
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
    let mut total_cards = 0;
    let mut card_count_map: HashMap<usize, usize> = HashMap::new();
    let mut i: usize = 0;
    for line in reader.lines() {
        let new_line = line?;
        let set: HashSet<&str> = HashSet::from_iter(new_line.split(":")
        .nth(1).unwrap().split("|").nth(0).unwrap().split(" ").filter(|&x| x != ""));
        let my_numbers = new_line.split("|")
        .nth(1).unwrap().split(" ").filter(|&x| x != "");
        let mut count = 0;
        for number in my_numbers {
            if set.contains(number) {
                count += 1;
            }
        }
        // Each time we have seen this card in the past will propogate
        let mut this_card = 1;
        if let Some(value) = card_count_map.get(&i) {
            this_card = *value + 1;
        }
        total_cards += this_card;
        // Go through and increment the number of cards for each card index.
        while count > 0 {
            *card_count_map.entry((i + count)).or_insert(0) += this_card;
            count -= 1;
        }
        
        i += 1;
    }
    println!("\tThe total sum of cards for part b is {}", total_cards);
    Ok(())
}
