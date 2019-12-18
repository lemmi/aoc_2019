use aoc2019::*;
use std::error::Error;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug,Default)]
struct Wire {
    segments: Vec<Point<isize>>
}

impl FromStr for Wire {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut w = Wire::default();
        w.segments.push(Point::new(0,0));

        for v in s.split(",") {
            let s: Box<Iterator<Item = (_,_)>> = match (&v[..1], (&v[1..]).parse::<isize>()?) {
                ("R", x) => Box::new(( 1..=x).map(|x| (0,  x))),
                ("L", x) => Box::new(( 1..=x).map(|x| (0, -x))),
                ("U", x) => Box::new(( 1..=x).map(|x| (-x, 0))),
                ("D", x) => Box::new(( 1..=x).map(|x| ( x, 0))),
                (_,_) => panic!("{:?} invalid segment", v),
            };

            let last = *w.segments.last().expect("can't be empty");
            w.segments.extend(s.map(|p| last + p.into()));
        }

        Ok(w)
    }
}

fn star1() -> Result<isize, Box<dyn Error>> {
    let ws: Vec<_> = lines("input")?.map(|l| l.parse::<Wire>()).collect::<Result<_,_>>()?;

    let mut i = ws.into_iter().map(|w| w.segments.into_iter().skip(1).collect::<HashSet<_>>());

    let s1 = i.next().unwrap();
    let s2 = i.next().unwrap();

    let nearest = s1.intersection(&s2).min_by(|p1, p2| (p1.manhatten()).cmp(&p2.manhatten()));
    
    Ok(nearest.unwrap().manhatten())
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let ws: Vec<_> = lines("input")?.map(|l| l.parse::<Wire>()).collect::<Result<_,_>>()?;

    let mut i = ws.into_iter().map(|w| w.segments.into_iter().enumerate().map(|(y,x)| (x,y)).skip(1).rev().collect::<HashMap<_,_>>());

    let s1 = i.next().unwrap();
    let s2 = i.next().unwrap();

    let k1 = s1.keys().collect::<HashSet<_>>();
    let k2 = s2.keys().collect::<HashSet<_>>();

    let inters = k1.intersection(&k2);
    let (_, steps) = inters.into_iter().map(|p| (p, s1[p] + s2[p])).min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    Ok(steps as isize)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("star1: {}", star1()?);
    println!("star2: {}", star2()?);
    Ok(())
}
