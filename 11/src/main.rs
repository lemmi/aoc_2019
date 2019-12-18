use aoc2019::intcode::Intcode;
use aoc2019::intcode::State;
use aoc2019::lines;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Add;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Vector<T>(T, T);

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self {
        Vector { 0: x, 1: y }
    }
}
impl<T: std::ops::Neg<Output = T>> Vector<T> {
    fn turn(self, t: isize) -> Self {
        match t {
            0 => Vector::new(self.1, -self.0),
            1 => Vector::new(-self.1, self.0),
            _ => panic!("unknown direction"),
        }
    }
}
impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(self.0 + other.0, self.1 + other.1)
    }
}

struct Robot {
    pos: Vector<isize>,
    dir: Vector<isize>,
    ic: Intcode,
}

fn paint(code: &str, panel: isize) -> Result<HashMap<Vector<isize>, isize>, Box<dyn Error>> {
    let mut hull: HashMap<Vector<isize>, isize> = HashMap::new();
    let mut r = Robot {
        pos: Vector::new(0, 0),
        dir: Vector::new(0, -1),
        ic: Intcode::from_str(code)?,
    };
    hull.insert(Vector::new(0, 0), panel);
    let mut total_out = 0;
    loop {
        let res = r.ic.run_input(&[hull.get(&r.pos).cloned().unwrap_or(0)]);
        for c in res.out() {
            match total_out % 2 {
                0 => {
                    hull.insert(r.pos, *c);
                }
                1 => {
                    r.dir = r.dir.turn(*c);
                    r.pos = r.pos + r.dir;
                }
                _ => unreachable! {},
            }
            total_out += 1;
        }
        if let State::Halt(_, _) = res {
            break;
        }
    }
    Ok(hull)
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let code = lines("input")?.next().unwrap();
    Ok(paint(&code, 0)?.len())
}

fn bb<T>(vs: T) -> (Vector<isize>, Vector<isize>)
where
    T: IntoIterator,
    T::Item: std::borrow::Borrow<Vector<isize>>,
{
    let mut vi = vs.into_iter();
    let mut min = *vi.next().unwrap().borrow();
    let mut max = min;

    for v in vi {
        min.0 = std::cmp::min(min.0, v.borrow().0);
        min.1 = std::cmp::min(min.1, v.borrow().1);
        max.0 = std::cmp::max(max.0, v.borrow().0);
        max.1 = std::cmp::max(max.1, v.borrow().1);
    }

    (min, max + Vector::new(1, 1))
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let code = lines("input")?.next().unwrap();
    let hull = paint(&code, 1)?;
    let (bbmin, bbmax) = bb(hull.keys());

    let mut paintjob = Vec::new();

    for _ in bbmin.1..bbmax.1 {
        paintjob.push(vec![' '; (bbmax.0 - bbmin.0) as usize]);
    }

    for (p, c) in hull {
        let x = (p.0 - bbmin.0) as usize;
        let y = (p.1 - bbmin.1) as usize;

        paintjob[y][x] = match c {
            1 => '#',
            0 => ' ',
            _ => unreachable!(),
        };
    }

    for l in paintjob {
        println!("{}", l.into_iter().collect::<String>());
    }
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
