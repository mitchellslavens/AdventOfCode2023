use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    rank: u32,
    bid: u32,
}

// I'm going to attempt to do this just by assigning a unique rank number to each hand
// First digit is the type of hand, rest of the digits are based on the card values ordered
pub fn day_07_a() -> std::io::Result<()> {
    let file = match File::open("inputs/input7.txt") {
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
    let mut total_sum = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for line in reader.lines() {
        let line= line?;
        let h = Hand {
            rank: determine_rank_number(line.split(" ").nth(0).unwrap()),
            bid: line.split(" ").nth(1).unwrap().parse::<u32>().unwrap(),
        };
        hands.push(h);
    }
    hands.sort_by_key(|hand| hand.rank);

    let mut i = 1;
    for hand in hands {
        total_sum += hand.bid * i;
        i += 1;
    }
    println!("\tThe total sum for part a is {}", total_sum);
    Ok(())
}

pub fn day_07_b() -> std::io::Result<()> {
    let file = match File::open("inputs/input7.txt") {
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
    let mut total_sum = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for line in reader.lines() {
        let line= line?;
        let h = Hand {
            rank: determine_rank_number_b(line.split(" ").nth(0).unwrap()),
            bid: line.split(" ").nth(1).unwrap().parse::<u32>().unwrap(),
        };
        hands.push(h);
    }
    hands.sort_by_key(|hand| hand.rank);

    let mut i = 1;
    for hand in hands {
        total_sum += hand.bid * i;
        i += 1;
    }
    println!("\tThe total sum for part b is {}", total_sum);
    Ok(())
}


// Both needs to determine the type of hand, and assign a value based on the cards and their positions
// High card = 1, Pair = 2, Two Pair = 3, Three of a Kind = 4, 
// Full House = 5, Four of a Kind = 6, Five of a Kind = 7
// Then, we will basically do a base 13 conversion to decimal... but a bit modified.
// 2 -> 1, 3 -> 2, 4 -> 3, 5 -> 4, 6 -> 5, 7 -> 6, 8 -> 7, 9 -> 8, T -> 9, J -> 10, Q -> 11, K -> 12, A -> 13
// The, we will have to multiply by 14 for each digit of higher order.
// So, we will have a unique rank for each unique ordering of cards.
// Then, we can use a very large number (larger than AAAAA which is 537823) to represent the hand types.
// For this, we will use 1000000 -> 7000000
fn determine_rank_number(hand: &str) -> u32 {
    let mut rank_number = 0;
    let mut multiplier = 1;
    let mut card_counts: HashMap<char, usize> = HashMap::new();
    for card in hand.chars().rev() {
        let card_value = match card {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0,
        };

        card_counts.insert(card, card_counts.get(&card).unwrap_or(&0) + 1);
        rank_number += multiplier * card_value;
        multiplier *= 14;
    }
    // Now, we need to determine the hand type using the card_counts map.
    let mut sorted_counts: Vec<_> = card_counts.values().collect();
    sorted_counts.sort_by(|a, b| b.cmp(a));
    match sorted_counts.as_slice() {
        [5] => rank_number += 7000000,
        [4, 1] => rank_number += 6000000,
        [3, 2] => rank_number += 5000000,
        [3, 1, 1] => rank_number += 4000000,
        [2, 2, 1] => rank_number += 3000000,
        [2, 1, 1, 1] => rank_number += 2000000,
        [1, 1, 1, 1, 1] => rank_number += 1000000,
        _ => rank_number += 0,
    }
    rank_number

    
}

// For part b... same thing but making J the bottom of the card values and using them as jokers.
fn determine_rank_number_b(hand: &str) -> u32 {
    let mut rank_number = 0;
    let mut multiplier = 1;
    let mut card_counts: HashMap<char, usize> = HashMap::new();
    let mut j_count = 0;
    for card in hand.chars().rev() {
        let card_value = match card {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0,
        };

        if card != 'J' {
            card_counts.insert(card, card_counts.get(&card).unwrap_or(&0) + 1);
        }
        else {
            j_count += 1;
        }
        
        rank_number += multiplier * card_value;
        multiplier *= 14;
    }
    // Now, we need to determine the hand type using the card_counts map.
    let mut sorted_counts: Vec<_> = card_counts.values().cloned().collect();
    sorted_counts.sort_by(|a, b| b.cmp(a));

    // We will always want to add the jokers to the most popular other card.
    if sorted_counts.is_empty() {
        sorted_counts.push(j_count);
    }
    else {
        sorted_counts[0] += j_count;
    }
    
    match sorted_counts.as_slice() {
        [5] => rank_number += 7000000,
        [4, 1] => rank_number += 6000000,
        [3, 2] => rank_number += 5000000,
        [3, 1, 1] => rank_number += 4000000,
        [2, 2, 1] => rank_number += 3000000,
        [2, 1, 1, 1] => rank_number += 2000000,
        [1, 1, 1, 1, 1] => rank_number += 1000000,
        _ => rank_number += 0,
    }
    rank_number

    
}