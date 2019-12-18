use aoc2019::{gcd, lines};
use std::collections::HashMap;
use std::error::Error;

type Loc = (isize, isize);
type Asteroids = Vec<Loc>;

fn read_map(filename: &str) -> Result<Asteroids, Box<dyn Error>> {
    let mut v = Vec::new();

    for (y, l) in lines(filename)?.enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                v.push((x as isize, y as isize));
            }
        }
    }

    Ok(v)
}

fn reduce((a, b): Loc) -> Loc {
    let div = gcd(a, b);
    (a / div, b / div)
}

fn los(asteroids: &Asteroids, base: &Loc) -> HashMap<Loc, Asteroids> {
    let mut alos = HashMap::new();
    asteroids
        .iter()
        .filter(|&a| a != base)
        .map(|a| (a.0 - base.0, a.1 - base.1))
        .map(|a| (reduce(a), a))
        .for_each(|(r, b)| alos.entry(r).or_insert(Vec::new()).push(b));

    for val in alos.values_mut() {
        val.sort_by_key(|(x, y)| x * x + y * y)
    }

    alos
}

fn find_best(asteroids: &Asteroids) -> (Loc, usize) {
    asteroids
        .into_iter()
        .map(|b| (*b, los(&asteroids, b).len()))
        .max_by_key(|&(_, n)| n)
        .unwrap()
}

fn angle(b: &Loc) -> f64 {
    let x = b.0 as f64;
    let y = b.1 as f64;

    (-x).atan2(y) + std::f64::consts::PI
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let asteroids = read_map("input")?;
    let (base, n) = find_best(&asteroids);
    println!("base at {:?} detects {} asteroids", base, n);

    Ok(n)
}
fn star2() -> Result<isize, Box<dyn Error>> {
    let asteroids = read_map("input")?;
    let (base, _) = find_best(&asteroids);
    let mut alos = los(&asteroids, &base);

    let mut shot = 0;
    let mut last;
    'outer: loop {
        let mut keys: Vec<_> = alos.keys().cloned().collect();
        keys.sort_by(|a, b| angle(a).partial_cmp(&angle(b)).unwrap());
        for k in keys {
            last = alos.get_mut(&k).unwrap().pop();
            shot += 1;
            //println!("{}: {:?}", shot, last);
            if shot == 200 {
                break 'outer;
            }
            //println!("{} {:?}: {:?}", angle(&k), k, v);
        }
        alos.retain(|_, v| v.len() > 0);
    }
    let last = last.unwrap();
    let last = (last.0 + base.0, last.1 + base.1);
    println!("last: {:?}", last);
    Ok(last.0 * 100 + last.1)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testbest() -> Result<(), Box<dyn Error>> {
        assert_eq!(((5, 8), 33), find_best(&read_map("test01")?));
        assert_eq!(((1, 2), 35), find_best(&read_map("test02")?));
        Ok(())
    }

    #[test]
    fn testangle() {
        assert!(angle(&(0, -1)) < angle(&(1, 0)));
        assert!(angle(&(0, -1)) < angle(&(0, 1)));
        assert!(angle(&(0, -1)) < angle(&(-1, 0)));
    }
}
