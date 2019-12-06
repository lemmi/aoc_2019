use std::io;
use aoc2019::lines;

fn fuel(mass: i32) -> i32 {
    let f = (mass / 3) - 2;
    if f < 0 {
        0
    } else {
        f
    }
}

fn fuelfuel(mass: i32) -> i32 {
    if mass <= 0 {
        0
    } else {
        let f = fuel(mass);
        f + fuelfuel(f)
    }
}

fn star1() -> io::Result<i32> {
    Ok(lines("input")?.map(|l| l.parse().unwrap()).map(fuel).sum())
}
fn star2() -> io::Result<i32> {
    Ok(lines("input")?.map(|l| l.parse().unwrap()).map(fuelfuel).sum())
}

fn main() -> io::Result<()> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
