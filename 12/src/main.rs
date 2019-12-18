use aoc2019::{lines,lcm};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Sub;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct V3<T>(T, T, T);

impl<T: Add<Output = T>> Add for V3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        V3 {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for V3<T> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        V3 {
            0: self.0 + other,
            1: self.1 + other,
            2: self.2 + other,
        }
    }
}

impl<T: Neg<Output = T>> Neg for V3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        V3 {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

impl<T: Not<Output = T>> Not for V3<T> {
    type Output = Self;
    fn not(self) -> Self::Output {
        V3 {
            0: !self.0,
            1: !self.1,
            2: !self.2,
        }
    }
}

impl<T: Sub<Output = T> + Neg<Output = T> + Add<Output = T>> Sub for V3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl V3<isize> {
    fn force(self, other: Self) -> Self {
        V3 {
            0: match self.0.cmp(&other.0) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
            1: match self.1.cmp(&other.1) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
            2: match self.2.cmp(&other.2) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
        }
    }

    fn energy(self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl FromStr for V3<isize> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('<').trim_end_matches('>');
        let mut iter = s.split(',').map(|f| f.split('=').last().unwrap().parse());

        Ok(V3 {
            0: iter.next().unwrap()?,
            1: iter.next().unwrap()?,
            2: iter.next().unwrap()?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Moon {
    p: V3<isize>,
    v: V3<isize>,
}

impl Hash for Moon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.energy().hash(state);
    }
}
impl Moon {
    fn apply_force(&self, other: &Self) -> V3<isize> {
        self.v + self.p.force(other.p)
    }
    fn energy(&self) -> isize {
        self.p.energy() * self.v.energy()
    }
    fn dim(&self, dim: usize) -> (isize,isize) {
        (
            [self.p.0, self.p.1, self.p.2][dim],
            [self.v.0, self.v.1, self.v.2][dim],
        )
    }
}
fn apply_force(moons: &mut [Moon]) {
    for i in 0..(moons.len() - 1) {
        for j in (i + 1)..moons.len() {
            moons[i].v = moons[i].apply_force(&moons[j]);
            moons[j].v = moons[j].apply_force(&moons[i]);
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for m in moons {
        m.p = m.p + m.v;
    }
}

fn step(moons: &mut [Moon]) {
    apply_force(moons);
    apply_velocity(moons);
}

fn total_energy(moons: &[Moon]) -> isize {
    moons.iter().map(Moon::energy).sum::<isize>()
}

fn parse_moons(filename: &str) -> Result<Vec<Moon>, Box<dyn Error>> {
    lines(filename)?
        .map(|l| {
            l.parse().map(|p| Moon {
                p: p,
                v: V3::default(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn state_in_dim(moons: &[Moon], dim: usize) -> Vec<(isize,isize)> {
    moons .into_iter() .map(|m| m.dim(dim)) .collect()
}

fn star1() -> Result<isize, Box<dyn Error>> {
    let mut moons = parse_moons("input")?;

    for _ in 0..1000 {
        step(&mut moons);
    }

    Ok(total_energy(&moons))
}

fn find_period(moons: &[Moon], dim: usize) -> (isize, isize) {
    let mut moons = moons.to_vec();
    let mut statedim = HashMap::new();

    for i in 0.. {
        if let Some(d) = statedim.insert(state_in_dim(&moons, dim), i) {
            return (d,i-d)
        }
        step(&mut moons);
    }
    unreachable!()
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let moons = parse_moons("input")?;

    let px = find_period(&moons, 0);
    let py = find_period(&moons, 1);
    let pz = find_period(&moons, 2);

    println!("x: {:?}", px);
    println!("y: {:?}", py);
    println!("z: {:?}", pz);

    Ok(lcm(lcm(px.1,py.1), pz.1))
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
