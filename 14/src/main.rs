use aoc2019::lines;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Substance {
    name: String,
    amount: usize,
}

impl FromStr for Substance {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut si = s.trim().split_whitespace();
        Ok(Substance {
            amount: si
                .next()
                .ok_or::<Box<dyn Error>>("Expected amount".into())?
                .parse()?,
            name: si
                .next()
                .ok_or::<Box<dyn Error>>("Expected name".into())?
                .to_string(),
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Reaction {
    product: Substance,
    reagents: Vec<Substance>,
}

impl FromStr for Reaction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut si = s.split("=>");
        let reagents = si
            .next()
            .ok_or::<Box<dyn Error>>("incomplete Reaction - no inputs".into())?
            .split(',')
            .map(|r| r.parse::<Substance>())
            .collect::<Result<Vec<_>, _>>()?;
        let product = si
            .next()
            .ok_or::<Box<dyn Error>>("incomplete reaction - no ouputs".into())?
            .parse::<Substance>()?;

        Ok(Reaction { reagents, product })
    }
}

#[derive(Default, Debug, Clone)]
struct Factory {
    reactions: HashMap<String, Reaction>,
    stock: HashMap<String, usize>,
    queue: Vec<(String, usize)>,
    ore: usize,
}

impl Factory {
    fn from_file(f: &str) -> Result<Factory, Box<dyn Error>> {
        Ok(Factory {
            reactions: lines(f)?
                .map(|l| l.parse::<Reaction>())
                .map(|r| r.map(|r| (r.product.name.clone(), r)))
                .collect::<Result<_, _>>()?,
            stock: HashMap::new(),
            queue: Vec::new(),
            ore: 0,
        })
    }

    fn need(&mut self, name: &str, amount: usize) {
        if name == "ORE" {
            self.ore += amount;
            return;
        }
        let mut a = amount;
        if let Some(s) = self.stock.get_mut(name) {
            a = a.saturating_sub(*s);
            *s = s.saturating_sub(amount);
        }

        if a == 0 {
            return;
        }

        let reac = &self.reactions[name];
        let n = (a - 1) / reac.product.amount + 1;
        *self.stock.entry(name.to_string()).or_insert(0) += n * reac.product.amount - a;

        for r in reac.reagents.iter() {
            self.queue.push((r.name.to_string(), r.amount * n));
        }
    }

    fn run(&mut self) {
        while !self.queue.is_empty() {
            let (name, amount) = self.queue.pop().unwrap();
            self.need(&name, amount);
        }
    }
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let mut f = Factory::from_file("input")?;
    f.need("FUEL", 1);
    f.run();
    Ok(f.ore)
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let available = 1_000_000_000_000usize;
    let mut cur = 1isize;
    let mut step = 1isize;
    let blueprint = Factory::from_file("input")?;
    loop {
        let mut f = blueprint.clone();
        f.need("FUEL", (cur + step) as usize);
        f.run();
        println!("{} + {} fuel need {} ore", cur, step, f.ore);
        if f.ore > available && step == 1{
            break;
        } else if f.ore > available {
            step /= 2;
        } else {
            cur += step;
            step *= 2;
        }
    }
    Ok(cur)
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
