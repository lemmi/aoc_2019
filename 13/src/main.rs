use aoc2019::{intcode, lines};
use std::error::Error;
use std::str::FromStr;

fn star1() -> Result<usize, Box<dyn Error>> {
    let mut ic = intcode::Intcode::from_str(&lines("input")?.next().unwrap())?;
    Ok(ic
        .run_input(&[])
        .out()
        .chunks(3)
        .filter(|t| t[2] == 2)
        .count())
}
fn star2() -> Result<isize, Box<dyn Error>> {
    let mut ic = intcode::Intcode::from_str(&lines("input")?.next().unwrap())?;
    *ic.mem(0) = 2;

    let mut display = 0;
    let mut ballx = 0;
    let mut joyx = 0;
    let mut input: &[isize] = &[];
    let mut out = intcode::State::Wait(vec![]);
    while !out.is_halt() {
        out = ic.run_input(input);
        for xyt in out.out().chunks(3) {
            match (xyt[0], xyt[1], xyt[2]) {
                (-1, 0, t) => {
                    display = t;
                }
                (x, _, 4) => {
                    ballx = x;
                }
                (x, _, 3) => {
                    joyx = x;
                }
                (_, _, _) => (),
            }
        }
        if ballx == joyx {
            input = &[0];
        }
        if ballx > joyx {
            input = &[1];
        }
        if ballx < joyx {
            input = &[-1];
        }
    }
    Ok(display)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
