use aoc2019::{intcode::Intcode, lines};
use std::error::Error;

fn star1() -> Result<isize, Box<dyn Error>> {
    let mut ic = lines("input")?.next().unwrap().parse::<Intcode>()?;
    ic.set(12, 2);
    ic.run()
}

fn star2() -> Result<isize, Box<dyn Error>> {
    for verb in 0..100 {
        for noun in 0..100 {
            let mut ic = lines("input")?.next().unwrap().parse::<Intcode>()?;
            ic.set(noun, verb);
            let res = ic.run()?;
            if res == 19_690_720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("star1: {}", star1()?);
    println!("star2: {}", star2()?);
    Ok(())
}
