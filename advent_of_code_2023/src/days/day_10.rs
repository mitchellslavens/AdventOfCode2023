use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_10_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input10.txt") {
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
    let mut grid: Vec<Vec<(char, i32)>> = Vec::new();
    let mut s_row = 0;
    let mut line_num = 0;
    let mut s_col = 0;
    for line in reader.lines() {
        let new_line = line?;
        if new_line.contains('S') {
            s_col = new_line.chars().position(|x| x == 'S').unwrap();
            s_row = line_num;
        }
        grid.push(new_line.chars().map(|c| (c, -1)).collect());
        line_num += 1;
    }
    
    let mut a_pos = (s_row, s_col);
    let mut b_pos = (s_row, s_col);
    let mut prev_a_pos = a_pos;
    let mut prev_b_pos = b_pos;
    // First, let's get these started, so the loop is easier. B cannot follow A.
    

    let MAX_ROW = grid.len();
    let MAX_COL = grid[0].len();
    let mut prev_a_pos = a_pos;
    let mut prev_b_pos = b_pos;
    let mut i = 1;
    while a_pos != b_pos || grid[a_pos.0][a_pos.1].0 == 'S' {

        // Advance a_pos (it should always find an unvisited position).
        let new_a_pos = find_valid_path(prev_a_pos, a_pos, &grid, MAX_ROW, MAX_COL, a_pos);
        grid[new_a_pos.0][new_a_pos.1].1 = i;


        // Advance b_pos (it can find a visited position at the very end).
        let mut new_b_pos = b_pos;
        if grid[b_pos.0][b_pos.1].0 == 'S' {
            new_b_pos = find_valid_path(prev_b_pos, b_pos, &grid, MAX_ROW, MAX_COL, new_a_pos);
        }
        else {
            new_b_pos = find_valid_path(prev_b_pos, b_pos, &grid, MAX_ROW, MAX_COL, (999999,999999));
        }
        grid[new_b_pos.0][new_b_pos.1].1 = i;
        
        prev_a_pos = a_pos;
        prev_b_pos = b_pos;
        a_pos = new_a_pos;
        b_pos = new_b_pos;

        grid[b_pos.0][b_pos.1].1 = i;
        i += 1;
    }

    println!("\tThe farthest position in the loop for part a is {}", i-1);
    Ok(())
}

fn find_valid_path(prev_pos: (usize, usize), current_pos: (usize, usize), grid: &Vec<Vec<(char, i32)>>, max_row: usize, max_col: usize, avoid: (usize, usize)) -> (usize, usize) {
    let curr_char = grid[current_pos.0][current_pos.1].0;
    // Try Going North
    if curr_char == '|' || curr_char == 'J' || curr_char == 'L' || curr_char == 'S' {
        if current_pos.0 != 0 && current_pos.0 - 1 != prev_pos.0  && current_pos.0 - 1 != avoid.0 {
            if is_valid_connector('S', grid[current_pos.0 - 1][current_pos.1].0) {
                return (current_pos.0 - 1, current_pos.1);
            }
        }
       
    }
    // Try South
    if curr_char == '|' || curr_char == '7' || curr_char == 'F' || curr_char == 'S' {
        if current_pos.0 + 1 <= max_row && current_pos.0 + 1 != prev_pos.0 && current_pos.0 + 1 != avoid.0 {  
            if is_valid_connector('N', grid[current_pos.0 + 1][current_pos.1].0) {
                return (current_pos.0 + 1, current_pos.1);
            }
        }

    }
    // Try East
    if curr_char == '-' || curr_char == 'L' || curr_char == 'F' || curr_char == 'S' {
        if current_pos.1 + 1 <= max_col && current_pos.1 + 1 != prev_pos.1 && current_pos.1 + 1 != avoid.1 {
            if is_valid_connector('W', grid[current_pos.0][current_pos.1 + 1].0) {
                return (current_pos.0, current_pos.1 + 1);
            }
        }
    }
    //Try West
    if curr_char == '-' || curr_char == 'J' || curr_char == '7' || curr_char == 'S' {
        if current_pos.1 != 0 && current_pos.1 -1 != prev_pos.1 && current_pos.1 - 1 != avoid.1 {
            if is_valid_connector('E', grid[current_pos.0][current_pos.1 - 1].0) {
                return (current_pos.0, current_pos.1 - 1);
            }
        }
    }
    println!("No valid path found");
    return (0, 0);
}

fn is_valid_connector(coming_from: char, connecting: char) -> bool {
    if coming_from == 'S' && (connecting == '|' || connecting == '7' || connecting == 'F') {
        return true;
    }
    if coming_from == 'N' && (connecting == '|' || connecting == 'J' || connecting == 'L') {
        return true;
    }
    if coming_from == 'E' && (connecting == '-' || connecting == 'L' || connecting == 'F') {
        return true;
    }
    if coming_from == 'W' && (connecting == '-' || connecting == 'J' || connecting == '7') {
        return true;
    }
    return false;
}