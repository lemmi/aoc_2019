use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

pub fn lines(filename: &str) -> io::Result<Box<dyn Iterator<Item = String>>>
{
    let f = File::open(filename)?;
    Ok(Box::new(
        io::BufReader::new(f)
            .lines()
            .map(|l| l.unwrap().trim().to_string()),
    ))
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point<T> {
    y: T,
    x: T,
}

impl<T> Point<T> {
    pub fn new(y: T, x: T) -> Self {
        return Point { y, x };
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

impl From<(isize, isize)> for Point<isize> {
    fn from(t: (isize, isize)) -> Self {
        Point::new(t.0, t.1)
    }
}

pub mod intcode {
    use std::error::Error;
    use std::str::FromStr;

    pub struct Intcode {
        mem: Vec<isize>,
        pc: isize,
        input: isize,
        output: Vec<isize>,
    }

    impl Intcode {
        pub fn op(&self) -> (isize, isize, isize, isize) {
            let op = self.mem[self.pc as usize + 0] as isize;
            (op % 100, op / 100 % 10, op / 1000 % 10, op / 10000 % 10)
        }
        pub fn src1(&self) -> isize {
            self.ind(1)
        }
        pub fn src2(&self) -> isize {
            self.ind(2)
        }
        pub fn dst(&mut self) -> &mut isize {
            self.ind_mut(3)
        }
        pub fn dir(&self, off: isize) -> isize {
            self.mem[off as usize]
        }
        pub fn dir_mut(&mut self, off: isize) -> &mut isize {
            &mut self.mem[off as usize]
        }
        pub fn ind(&self, off: isize) -> isize {
            self.mem[self.mem[(self.pc + off) as usize] as usize]
        }
        pub fn ind_mut(&mut self, off: isize) -> &mut isize {
            let idx = self.mem[(self.pc + off) as usize] as usize;
            &mut self.mem[idx]
        }
        pub fn param(&self, off: isize, mode: isize) -> isize {
            let imm = self.mem[(self.pc + off) as usize];
            match mode {
                0 => self.mem[imm as usize],
                1 => imm,
                x => panic!("unkown parameter mode {}", x),
            }
        }
        pub fn step(&mut self) -> Option<isize> {
            //println!("{:?}", &self.mem[self.pc as usize .. self.pc as usize + 4]);
            match self.op() {
                (1, p1, p2, _) => {
                    *self.ind_mut(3) = self.param(1, p1) + self.param(2, p2);
                    self.pc += 4;
                    None
                },
                (2, p1, p2, _) => {
                    *self.ind_mut(3) = self.param(1, p1) * self.param(2, p2);
                    self.pc += 4;
                    None
                },
                (3, _, _, _) => {
                    *self.ind_mut(1) = self.input;
                    self.pc += 2;
                    None
                },
                (4, p1, _, _) => {
                    self.output.push(self.param(1,p1));
                    self.pc += 2;
                    None
                },
                (5, p1, p2, _) => {
                    match self.param(1,p1) {
                        0 => { self.pc += 3 },
                        _ => { self.pc = self.param(2,p2) },
                    }
                    None
                },
                (6, p1, p2, _) => {
                    match self.param(1,p1) {
                        0 => { self.pc = self.param(2,p2) },
                        _ => { self.pc += 3 },
                    }
                    None
                },
                (7, p1, p2, _) => {
                    *self.ind_mut(3) = (self.param(1,p1) < self.param(2,p2)) as isize;
                    self.pc += 4;
                    None
                },
                (8, p1, p2, _) => {
                    *self.ind_mut(3) = (self.param(1,p1) == self.param(2,p2)) as isize;
                    self.pc += 4;
                    None
                },
                (99, _, _, _) => Some(self.mem[0]),
                (x, _, _, _) => panic!("Invalid opcode {}, pc: {}", x, self.pc),
            }
        }
        pub fn run(&mut self) -> Result<isize, Box<Error>> {
            loop {
                match self.step() {
                    None => (),
                    Some(x) => return Ok(x),
                }
            }
        }
        pub fn run_input(&mut self, input: isize) -> Result<&[isize], Box<Error>> {
            self.input = input;
            loop {
                match self.step() {
                    None => (),
                    Some(_) => return Ok(&self.output),
                }
            }
        }

        pub fn set(&mut self, noun: isize, verb: isize) {
            self.mem[1] = noun;
            self.mem[2] = verb;
        }
    }
    impl FromStr for Intcode {
        type Err = Box<Error>;
        fn from_str(src: &str) -> Result<Self, Self::Err> {
            let mem: Result<Vec<isize>, _> = src.split(",").map(|n| n.parse()).collect();
            Ok(Intcode {
                mem: mem?,
                pc: 0,
                input: 0,
                output: Vec::new(),
            })
        }
    }
}
