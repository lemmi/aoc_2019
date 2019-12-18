use aoc2019::lines;
use std::error::Error;
use std::iter::successors;
use std::ops::Range;

fn pattern_range(l: usize, e: usize, s: usize) -> Vec<(Range<usize>, i32)> {
    let mut ret = Vec::new();
    let mut sign = 1;
    for n in 1.. {
        let n = 2 * n - 1;
        let start = n * (e + 1) - 1 - s;
        let end = (n + 1) * (e + 1) - 1 - s;
        if start >= l - s {
            break;
        }
        ret.push((start..end.min(l - s), sign));
        sign *= -1;
        if end > l - s {
            break;
        }
    }
    ret
}

fn patterns(l: usize, s: usize) -> Vec<Vec<(Range<usize>, i32)>> {
    (s..l).map(|e| pattern_range(l, e, s)).collect()
}

fn phase(v: &[i32], ps: &[Vec<(Range<usize>, i32)>]) -> Vec<i32> {
    ps.iter()
        .map(|p| {
            p.iter()
                .cloned()
                .map(|(r, sgn)| v[r].iter().sum::<i32>() * sgn)
                .sum::<i32>()
        })
        .map(|d| (d % 10).abs())
        .collect()
}

fn from_file(f: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    lines(f)?
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as i32)
                .ok_or("failed to parse digit".into())
        })
        .collect()
}

fn digits_to_num(v: &[i32]) -> i32 {
    let mut ret = 0;
    for d in v.iter() {
        ret = 10 * ret + d;
    }
    ret
}

fn star1() -> Result<i32, Box<dyn Error>> {
    let v = from_file("input")?;
    let ps = patterns(v.len(), 0);

    let res = successors(Some(v), |v| Some(phase(v, &ps)))
        .skip(100)
        .next()
        .unwrap()[..8]
        .to_vec();
    Ok(digits_to_num(&res))
}

fn phase2(v: &[i32]) -> Vec<i32> {
    let mut ret: Vec<i32> = v
        .iter()
        .rev()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .map(|d| (d % 10).abs())
        .collect();
    ret.reverse();
    ret
}

fn star2() -> Result<i32, Box<dyn Error>> {
    let v = from_file("input")?;
    let l = v.len() * 10_000;
    let off = digits_to_num(&v[..7]) as usize;

    // make actually sure we can just do a rolling sum from the end:
    let ps = patterns(l, off);
    for p in &ps {
        assert!(p.len() == 1);
        let (r, sgn) = &p[0];
        assert!(*sgn == 1);
        assert!(r.end == l - off);
    }

    let v: Vec<i32> = v.iter().cycle().take(l).skip(off).cloned().collect();
    let res = successors(Some(v), |v| Some(phase2(v)))
        .skip(100)
        .next()
        .unwrap()[..8]
        .to_vec();
    Ok(digits_to_num(&res))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {:?}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
