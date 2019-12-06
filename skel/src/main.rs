use aoc2019::lines;
use std::error::Error;

fn star1() -> Result<usize, Box<Error>> {
    lines("input")?;
    Ok(0)
}
fn star2() -> Result<usize, Box<Error>> {
    lines("input")?;
    Ok(0)
}

fn main() -> Result<(), Box<Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
