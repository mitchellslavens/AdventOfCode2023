mod days;

fn main() -> std::io::Result<()> {
    let _ = days::day_01_a();
    let _ = days::day_01_b(); // 54489 is wrong
    Ok(())
}