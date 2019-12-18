use aoc2019::intcode::Intcode;
use std::error::Error;

fn scaffolding_from_file(f: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut ic = Intcode::from_file(f)?;
    let s: Vec<char> = ic
        .run_input(&[])
        .out()
        .iter()
        .map(|c| *c as u8 as char)
        .collect();
    Ok(s.split(|&c| c == '\n')
        .map(|v| v.to_vec())
        .filter(|v| v.len() != 0)
        .collect())
}

fn alignment_parameters(sc: &[Vec<char>]) -> usize {
    let h = sc.len();
    let w = sc[0].len();

    let mut sum = 0;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if sc[y][x] == '#'
                && sc[y - 1][x] == '#'
                && sc[y + 1][x] == '#'
                && sc[y][x - 1] == '#'
                && sc[y][x + 1] == '#'
            {
                sum += y * x;
            }
        }
    }
    sum
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let s = scaffolding_from_file("input")?;
    for l in &s {
        println!("{}", l.iter().collect::<String>());
    }
    Ok(alignment_parameters(&s))
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let input = "A,A,B,C,B,C,B,C,B,A
L,10,L,8,R,8,L,8,R,6
R,6,R,8,R,8
R,6,R,6,L,8,L,10
n
";
    let input: Vec<isize> = input.chars().map(|c| c as isize).collect();
    let mut ic = Intcode::from_file("input")?;
    *ic.mem(0) = 2;
    Ok(*ic.run_input(&input).out().last().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
