use aoc2019::lines;
use aoc2019::intcode::Intcode;
use std::str::FromStr;
use std::error::Error;
use std::sync::{Arc,mpsc};
use std::thread;

use itertools::Itertools;

fn star1() -> Result<isize, Box<Error>> {
    let code = lines("input")?.next().unwrap();

    let mut max = 0;

    for phases in (0..=4).permutations(5) {
        let mut input = 0;
        for phase in phases {
            let mut amp = Intcode::from_str(&code)?;
            input = amp.run_input(&[phase, input])?.recv()?;
        }

        if input > max {
            max = input;
        }
    }

    Ok(max)
}

fn star2() -> Result<isize, Box<Error>> {
    let code = Arc::new(lines("input")?.next().unwrap());

    let mut max = 0;
    for phases in (5..=9).permutations(5) {
        let mut amps = Vec::new();

        let (s0, ra) = mpsc::channel();
        let (sa, rb) = mpsc::channel();
        let (sb, rc) = mpsc::channel();
        let (sc, rd) = mpsc::channel();
        let (sd, re) = mpsc::channel();
        let (se, r0) = mpsc::channel();
        s0.send(phases[0])?;
        s0.send(0)?;
        sa.send(phases[1])?;
        sb.send(phases[2])?;
        sc.send(phases[3])?;
        sd.send(phases[4])?;
        let mut amp = Intcode::from_str(&code).unwrap();
        amps.push(thread::spawn(move || { amp.run_channel(ra, sa); }));
        let mut amp = Intcode::from_str(&code).unwrap();
        amps.push(thread::spawn(move || { amp.run_channel(rb, sb); }));
        let mut amp = Intcode::from_str(&code).unwrap();
        amps.push(thread::spawn(move || { amp.run_channel(rc, sc); }));
        let mut amp = Intcode::from_str(&code).unwrap();
        amps.push(thread::spawn(move || { amp.run_channel(rd, sd); }));
        let mut amp = Intcode::from_str(&code).unwrap();
        amps.push(thread::spawn(move || { amp.run_channel(re, se); }));

        for out in r0 {
            match s0.send(out) {
                Ok(()) => (),
                Err(_) => {
                    if max < out {
                        max = out
                    }
                    break;
                }
            }
        }


        for a in amps.into_iter() {
            a.join().unwrap();
        }
    }
    Ok(max)
}

fn main() -> Result<(), Box<Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
