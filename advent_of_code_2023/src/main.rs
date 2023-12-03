mod days;

fn main() -> std::io::Result<()> {
    println!("Day1:");
    let _ = days::day_01_a();
    let _ = days::day_01_b();
    println!("Day2:");
    let _ = days::day_02_a();
    let _ = days::day_02_b();
    println!("Day3:");
    let _ = days::day_03_a();
    let _ = days::day_03_b();
    Ok(())
} 