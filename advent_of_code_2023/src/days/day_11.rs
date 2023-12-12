use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Reverse;

pub fn day_11_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input11.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found");
                return Ok(());
            }
            _ => return Err(error),
        },
    };
    let reader = BufReader::new(file);
    // Read in each line, record positions of galaxies.
    // If line has no galaxies, add another line of all '.' right away.
    // Also, maintain what x coordinates have been found. At the end, we can add extra '.' columns.
    let mut row = 0;
    let mut col: usize = 0;
    let mut no_galaxy_cols = HashSet::new();
    let mut galaxy_coords: Vec<(i32, i32)> = Vec::new();
    for line in reader.lines() {
        col = 0;
        let line = line?;
        let mut galaxy_found = false;
        for c in line.chars() {
            if row == 0 {
                no_galaxy_cols.insert(col);
            }
            if c == '#' {
                // Found a galaxy!
                galaxy_found = true;
                no_galaxy_cols.remove(&col);
                galaxy_coords.push((row as i32, col as i32));
            }
            col += 1;
        }
        if !galaxy_found {
            row += 1;
        }
        row += 1;
    }

    for i in 0..galaxy_coords.len() {
        galaxy_coords[i].1 += no_galaxy_cols.clone().into_iter().filter(|w| (*w as i32) < (galaxy_coords[i].1)).count() as i32;
    }

    let sum = galaxy_coords.iter()
        .enumerate()
        .flat_map(|(i, a)| galaxy_coords[i+1..].iter().map(move |b| (a.0 - b.0).abs() + (a.1 - b.1).abs()))
        .sum::<i32>();
    
    println!("\tThe total sum for part a is {}", sum);
    Ok(())
}

pub fn day_11_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input11.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found");
                return Ok(());
            }
            _ => return Err(error),
        },
    };
    let reader = BufReader::new(file);
    // Read in each line, record positions of galaxies.
    // If line has no galaxies, add another line of all '.' right away.
    // Also, maintain what x coordinates have been found. At the end, we can add extra '.' columns.
    let mut row = 0;
    let mut col: usize = 0;
    let mut no_galaxy_cols = HashSet::new();
    let mut galaxy_coords: Vec<(i64, i64)> = Vec::new();
    for line in reader.lines() {
        col = 0;
        let line = line?;
        let mut galaxy_found = false;
        for c in line.chars() {
            if row == 0 {
                no_galaxy_cols.insert(col);
            }
            if c == '#' {
                // Found a galaxy!
                galaxy_found = true;
                no_galaxy_cols.remove(&col);
                galaxy_coords.push((row as i64, col as i64));
            }
            col += 1;
        }
        if !galaxy_found {
            row += 999_999;
        }
        row += 1;
    }

    for i in 0..galaxy_coords.len() {
        galaxy_coords[i].1 += 999_999 * no_galaxy_cols.clone().into_iter().filter(|w| (*w as i64) < (galaxy_coords[i].1)).count() as i64;
    }

    let sum = galaxy_coords.iter()
        .enumerate()
        .flat_map(|(i, a)| galaxy_coords[i+1..].iter().map(move |b| (a.0 - b.0).abs() + (a.1 - b.1).abs()))
        .sum::<i64>();
    
    println!("\tThe total sum for part b is {}", sum);
    Ok(())
}
