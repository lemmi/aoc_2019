use aoc2019::lines;
use std::error::Error;

struct Intcode {
    mem: Vec<isize>,
    pc: isize,
}

impl Intcode {
    fn op(&self) -> isize {
        self.mem[self.pc as usize + 0]
    }
    fn src1(&self) -> isize {
        self.mem[self.mem[self.pc as usize + 1] as usize]
    }
    fn src2(&self) -> isize {
        self.mem[self.mem[self.pc as usize + 2] as usize]
    }

    fn dst(&mut self) -> &mut isize {
        let idx = self.mem[self.pc as usize + 3] as usize;
        &mut self.mem[idx]
    }
    fn step(&mut self) -> Option<isize> {
        //println!("{:?}", &self.mem[self.pc as usize .. self.pc as usize + 4]);
        match self.op() {
            1 => {
                *self.dst() = self.src1() + self.src2();
                self.pc += 4;
                None
            },
            2 => {
                *self.dst() = self.src1() * self.src2();
                self.pc += 4;
                None
            },
            99 => {
                Some(self.mem[0])
            },
            x => {
                panic!("Invalid opcode {}, pc: {}", x, self.pc)
            }
        }
    }

    fn run(&mut self) -> Result<isize,Box<Error>> {
        loop {
            match self.step() {
                None => (),
                Some(x) => return Ok(x),
            }
        }
    }

    fn set(&mut self, noun: isize, verb: isize) {
        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    fn read_prog(src: &str) -> Result<Intcode, Box<Error>> {
        let mem: Result<Vec<isize>,_> = lines::<String>(src)?.next().unwrap().split(",").map(|n| n.parse()).collect();
        Ok(Intcode{mem: mem?, pc: 0})
    }
}

fn star1() -> Result<isize, Box<Error>> {
    let mut ic = Intcode::read_prog("input")?;
    ic.set(12,2);
    ic.run()
}

fn star2() -> Result<isize, Box<Error>> {
    for verb in 0..100 {
        for noun in 0..100 {
            let mut ic = Intcode::read_prog("input")?;
            ic.set(noun,verb);
            let res = ic.run()?;
            if res == 19690720 {
                return Ok(100*noun + verb)
            }
        }
    }
    Ok(0)
}

fn main() -> Result<(), Box<Error>> {
    println!("star1: {}", star1()?);
    println!("star2: {}", star2()?);
    Ok(())
}
