use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

pub fn lines(filename: &str) -> io::Result<Box<dyn Iterator<Item = String>>> {
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

pub fn gcd(a: isize, b: isize) -> isize {
    let a = a.abs();
    let b = b.abs();
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(a: isize, b: isize) -> isize {
    (a * b).abs() / gcd(a, b)
}

pub mod intcode {
    use std::error::Error;
    use std::str::FromStr;

    pub struct Intcode {
        mem: Vec<isize>,
        pc: isize,
        base: isize,
    }

    enum InstrState {
        Run,
        Wait,
        Output(isize),
        Halt(isize),
    }

    #[derive(Debug, PartialEq)]
    pub enum State<T> {
        Wait(T),
        Halt(T, isize),
    }

    impl<T> State<T> {
        pub fn out(&self) -> &T {
            match self {
                State::Wait(t) => t,
                State::Halt(t, _) => t,
            }
        }
        pub fn is_halt(&self) -> bool {
            match self {
                State::Wait(_) => false,
                State::Halt(_, _) => true,
            }
        }
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
                self.mem.resize(immaddr + 1, 0);
            }
            let addr = match mode {
                0 => self.mem[immaddr] as usize,
                1 => immaddr,
                2 => (self.mem[immaddr] + self.base) as usize,
                x => panic!("unknown parameter mode {}", x),
            };
            if addr >= self.mem.len() {
                self.mem.resize(addr + 1, 0);
            }
            addr
        }
        pub fn param(&mut self, off: isize, mode: isize) -> isize {
            let addr = self.param_addr(off, mode);
            self.mem[addr]
        }
        pub fn param_mut(&mut self, off: isize, mode: isize) -> &mut isize {
            let addr = self.param_addr(off, mode);
            &mut self.mem[addr]
        }
        fn step(&mut self, input: &mut dyn Iterator<Item = isize>) -> InstrState {
            match self.op() {
                (1, p1, p2, p3) => {
                    *self.param_mut(3, p3) = self.param(1, p1) + self.param(2, p2);
                    self.pc += 4;
                    InstrState::Run
                }
                (2, p1, p2, p3) => {
                    *self.param_mut(3, p3) = self.param(1, p1) * self.param(2, p2);
                    self.pc += 4;
                    InstrState::Run
                }
                (3, p1, _, _) => {
                    if let Some(input) = input.next() {
                        *self.param_mut(1, p1) = input;
                        self.pc += 2;
                        InstrState::Run
                    } else {
                        InstrState::Wait
                    }
                }
                (4, p1, _, _) => {
                    let data = self.param(1, p1);
                    self.pc += 2;
                    InstrState::Output(data)
                }
                (5, p1, p2, _) => {
                    match self.param(1, p1) {
                        0 => self.pc += 3,
                        _ => self.pc = self.param(2, p2),
                    }
                    InstrState::Run
                }
                (6, p1, p2, _) => {
                    match self.param(1, p1) {
                        0 => self.pc = self.param(2, p2),
                        _ => self.pc += 3,
                    }
                    InstrState::Run
                }
                (7, p1, p2, p3) => {
                    *self.param_mut(3, p3) = (self.param(1, p1) < self.param(2, p2)) as isize;
                    self.pc += 4;
                    InstrState::Run
                }
                (8, p1, p2, p3) => {
                    *self.param_mut(3, p3) = (self.param(1, p1) == self.param(2, p2)) as isize;
                    self.pc += 4;
                    InstrState::Run
                }
                (9, p1, _, _) => {
                    self.base += self.param(1, p1) as isize;
                    self.pc += 2;
                    InstrState::Run
                }
                (99, _, _, _) => InstrState::Halt(self.mem[0]),
                (x, _, _, _) => panic!("Invalid opcode {}, pc: {}", x, self.pc),
            }
        }
        pub fn run(&mut self) -> Result<isize, Box<dyn Error>> {
            loop {
                match self.step(&mut std::iter::empty()) {
                    InstrState::Run => (),
                    InstrState::Wait => panic!("input needed"),
                    InstrState::Output(_) => (),
                    InstrState::Halt(x) => return Ok(x),
                }
            }
        }
        pub fn run_input(&mut self, input: &[isize]) -> State<Vec<isize>> {
            let mut out = Vec::new();
            let mut in_iter = input.into_iter().cloned();
            loop {
                match self.step(&mut in_iter) {
                    InstrState::Run => (),
                    InstrState::Wait => {
                        return State::Wait(std::mem::replace(&mut out, Vec::new()));
                    }
                    InstrState::Output(o) => out.push(o),
                    InstrState::Halt(h) => {
                        return State::Halt(std::mem::replace(&mut out, Vec::new()), h)
                    }
                }
            }
        }
        pub fn set(&mut self, noun: isize, verb: isize) {
            self.mem[1] = noun;
            self.mem[2] = verb;
        }
        pub fn mem(&mut self, idx: usize) -> &mut isize {
            &mut self.mem[idx]
        }
        pub fn from_file(f: &str) -> Result<Self, Box<dyn Error>> {
            super::lines(f)?
                .next()
                .ok_or("no code available".to_string())?
                .parse()
        }
    }
    impl FromStr for Intcode {
        type Err = Box<dyn Error>;
        fn from_str(src: &str) -> Result<Self, Self::Err> {
            let mem: Result<Vec<isize>, _> = src.split(",").map(|n| n.parse()).collect();
            Ok(Intcode {
                mem: mem?,
                pc: 0,
                base: 0,
            })
        }
    }
}
