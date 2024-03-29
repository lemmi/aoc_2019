use aoc2019::lines;
use std::error::Error;

fn star1() -> Result<usize, Box<dyn Error>> {
    lines("input")?;
    Ok(0)
}
fn star2() -> Result<usize, Box<dyn Error>> {
    lines("input")?;
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
