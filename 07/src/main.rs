use aoc2019::intcode::Intcode;
use aoc2019::intcode::State;
use aoc2019::lines;
use std::error::Error;
use std::str::FromStr;

use itertools::Itertools;

fn star1() -> Result<isize, Box<dyn Error>> {
    let code = lines("input")?.next().unwrap();

    let mut max = 0;

    for phases in (0..=4).permutations(5) {
        let mut input = 0;
        for phase in phases {
            let mut amp = Intcode::from_str(&code)?;
            input = *amp.run_input(&[phase, input]).out().last().unwrap();
        }

        if input > max {
            max = input;
        }
    }

    Ok(max)
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let code = lines("input")?.next().unwrap();

    let mut max = 0;
    for phases in (5..=9).permutations(5) {
        let mut amps = (0..5)
            .into_iter()
            .map(|_| Intcode::from_str(&code))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        for (phase, amp) in phases.into_iter().zip(&mut amps) {
            assert_eq!(State::Wait(vec![]), amp.run_input(&[phase]));
        }

        let mut input = vec![0];
        let mut halted = 0;
        while halted == 0 {
            for amp in &mut amps {
                match amp.run_input(&input) {
                    State::Wait(o) => {
                        input = o;
                    }
                    State::Halt(o, _) => {
                        input = o;
                        halted += 1;
                    }
                }
            }
        }

        max = std::cmp::max(max, input.pop().unwrap())
    }
    Ok(max)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
