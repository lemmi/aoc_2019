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
    use std::sync::mpsc;

    pub struct Intcode {
        mem: Vec<isize>,
        pc: isize,
        base: isize,
        input: mpsc::Receiver<isize>,
        output: mpsc::Sender<isize>,
    }

    impl Intcode {
        fn op(&self) -> (isize, isize, isize, isize) {
            let op = self.mem[self.pc as usize + 0] as isize;
            (op % 100, op / 100 % 10, op / 1000 % 10, op / 10000 % 10)
        }
        pub fn ind_mut(&mut self, off: isize) -> &mut isize {
            let idx = self.mem[(self.pc + off) as usize] as usize;
            &mut self.mem[idx]
        }
        pub fn param_addr(&mut self, off: isize, mode: isize) -> usize {
            let immaddr = (self.pc + off) as usize;
            if immaddr >= self.mem.len() {
                self.mem.resize(immaddr+1,0);
            }
            let addr = match mode {
                0 => self.mem[immaddr] as usize,
                1 => immaddr,
                2 => (self.mem[immaddr] + self.base) as usize,
                x => panic!("unknown parameter mode {}", x),
            };
            if addr >= self.mem.len() {
                self.mem.resize(addr+1,0);
            }
            addr
        }
        pub fn param(&mut self, off: isize, mode: isize) -> isize {
            let addr = self.param_addr(off,mode);
            self.mem[addr]
        }
        pub fn param_mut(&mut self, off: isize, mode: isize) -> &mut isize {
            let addr = self.param_addr(off,mode);
            &mut self.mem[addr]
        }
        pub fn step(&mut self) -> Option<isize> {
            //println!("{:?}", &self.mem[self.pc as usize .. self.pc as usize + 4]);
            match self.op() {
                (1, p1, p2, p3) => {
                    *self.param_mut(3,p3) = self.param(1, p1) + self.param(2, p2);
                    self.pc += 4;
                    None
                },
                (2, p1, p2, p3) => {
                    *self.param_mut(3, p3) = self.param(1, p1) * self.param(2, p2);
                    self.pc += 4;
                    None
                },
                (3, p1, _, _) => {
                    *self.param_mut(1,p1) = self.input.recv().unwrap();
                    self.pc += 2;
                    None
                },
                (4, p1, _, _) => {
                    let data = self.param(1,p1);
                    self.output.send(data).unwrap();
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
                (7, p1, p2, p3) => {
                    *self.param_mut(3,p3) = (self.param(1,p1) < self.param(2,p2)) as isize;
                    self.pc += 4;
                    None
                },
                (8, p1, p2, p3) => {
                    *self.param_mut(3,p3) = (self.param(1,p1) == self.param(2,p2)) as isize;
                    self.pc += 4;
                    None
                },
                (9, p1, _, _) => {
                    self.base += self.param(1,p1) as isize;
                    self.pc += 2;
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
        pub fn run_channel(&mut self, input: mpsc::Receiver<isize>, output: mpsc::Sender<isize>) {
            self.input = input;
            self.output = output;
            self.base = 0;
            loop {
                match self.step() {
                    None => (),
                    Some(_) => return,
                }
            }
        }
        pub fn run_input(&mut self, input: &[isize]) -> Result<Vec<isize>, Box<Error>> {
            let (si, ri) = mpsc::channel();
            let (so, ro) = mpsc::channel();
            self.input = ri;
            self.output = so;
            self.base = 0;
            for i in input {
                si.send(*i)?;
            }
            loop {
                match self.step() {
                    None => (),
                    Some(_) => return Ok(ro.try_iter().collect())
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
            let (s,r) = mpsc::channel();
            Ok(Intcode {
                mem: mem?,
                pc: 0,
                base: 0,
                input: r,
                output: s,
            })
        }
    }
}
