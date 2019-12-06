use std::io;

fn digits(x: isize) -> Vec<u8> {
    let mut n = x;
    let mut ret = Vec::new();

    while n > 0 {
        let r = n / 10;
        let d = n - r * 10;
        ret.push(d as u8);
        n = r;
    }

    ret.reverse();
    ret
}

fn checklen(ds: &[u8]) -> bool {
    ds.len() == 6
}
fn checkdoubles<T: PartialEq>(ds: &[T]) -> bool {
    ds.windows(2).any(|w| w[0] == w[1])
}
fn checkmonotonic(ds: &[u8]) -> bool {
    ds.windows(2).all(|w| w[0] <= w[1])
}

fn check1(n: isize) -> bool {
    let ds = digits(n);
    checklen(&ds) && checkdoubles(&ds) && checkmonotonic(&ds)
}

fn checkdoubles2(ds: &[u8]) -> bool {
    let mut last = 10;
    let mut count = 0;

    for c in ds {
        if last == *c {
            count += 1
        }
        if last != *c {
            if count == 2 {
                break;
            }
            count = 1
        }
        last = *c
    }
    count == 2
}

fn check2(n: isize) -> bool {
    let ds = digits(n);
    checklen(&ds) && checkdoubles2(&ds) && checkmonotonic(&ds)
}

fn star1() -> io::Result<usize> {
    Ok((197_487..=673_251).filter(|x| check1(*x as isize)).count())
}
fn star2() -> io::Result<usize> {
    Ok((197_487..=673_251).filter(|x| check2(*x as isize)).count())
}

fn main() -> io::Result<()> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check1, check2};

    #[test]
    fn test_check1() {
        assert!(check1(111111));
        assert!(check1(122345));
        assert!(!check1(223450));
        assert!(!check1(123789));
        assert!(!check1(11111));
    }
    #[test]
    fn test_check2() {
        assert!(!check2(111111));
        assert!(check2(112233));
        assert!(check2(111122));
        assert!(check2(122345));
        assert!(!check2(223450));
        assert!(!check2(123789));
        assert!(!check2(11111));
        assert!(!check2(123444));
    }
}
