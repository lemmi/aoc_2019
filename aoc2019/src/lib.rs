use std::io;
use std::io::BufRead;
use std::fs::File;
use std::ops::Add;

pub fn lines<T>(filename: &str) -> io::Result<Box<dyn Iterator<Item=T>>>
    where T: std::str::FromStr,
    T::Err: std::fmt::Debug
{
    let f = File::open(filename)?;
    Ok(Box::new(io::BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())))
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point<T> {
    y: T,
    x: T,
}

impl<T> Point<T> {
    pub fn new(y: T, x: T) -> Self {
        return Point{y,x}
    }

}

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}
impl Point<isize> {
    pub fn manhatten(self) -> isize {
        self.y.abs() + self.x.abs()
    }
}

impl From<(isize,isize)> for Point<isize> {
    fn from(t: (isize,isize)) -> Self {
        Point::new(t.0, t.1)
    }
}
