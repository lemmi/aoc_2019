use aoc2019::lines;
use aoc2019::intcode::Intcode;
use std::str::FromStr;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug,Default)]
struct Loc (isize, isize);
impl Loc {
    fn dir(&self, dir:isize) -> Self {
        match dir {
            1 => Loc{0: self.0,  1: self.1-1},
            2 => Loc{0: self.0,  1: self.1+1},
            3 => Loc{0: self.0-1,1: self.1},
            4 => Loc{0: self.0+1,1: self.1},
            x => panic!("unknown direction {}", x),
        }
    }
}

fn reverse(dir: isize) -> isize {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        x => panic!("unknown direction {}", x),
    }
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct WayPoint {
    loc: Loc,
    neigh: Vec<isize>,
    dist: usize,
    oxygen: bool,
    parent_dir: isize,
}

impl WayPoint {
    fn parent(&self) -> Loc {
        self.loc.dir(self.parent_dir)
    }
}

#[derive(Debug,Default)]
struct Map(HashMap<Loc, WayPoint>);

struct Robot {
    ic: Intcode,
    path: Vec<Loc>,
    map: Map,
}

impl Robot {
    fn from_file(f: &str) -> Result<Self,Box<dyn Error>> {
        let mut map = Map::default();
        map.0.insert(Loc::default(), WayPoint::default());
        Ok(Robot{
            ic: Intcode::from_str(&lines(f)?.next().unwrap())?,
            path: Vec::new(),
            map: map,
        })
    }
    fn peek(&mut self, dir: isize) -> isize {
        let tile = self.mv(dir);
        if tile > 0 {
            self.back(dir);
        }
        tile
    }
    fn back(&mut self, dir: isize) {
        let tile = *self.ic.run_input(&[reverse(dir)]).out().first().unwrap();
        assert!(tile > 0);
        self.path.pop();
    }
    fn mv(&mut self, dir: isize) -> isize {
        let tile = *self.ic.run_input(&[dir]).out().first().unwrap();
        if tile > 0 {
            self.path.push(self.pos().dir(dir));
        }
        tile
    }
    fn find_path(&self, mut loc: Loc) -> Vec<isize> {
        let mut ret = Vec::new();
        while loc != Loc::default() {
            let wp = self.map.0.get(&loc).unwrap();
            ret.push(reverse(wp.parent_dir));
            loc = wp.parent();
        }
        ret.reverse();
        ret
    }
    fn follow_path(&mut self, path: &[isize]) {
        for dir in path.into_iter() {
            self.mv(*dir);
        }
    }
    fn follow_path_rev(&mut self, path: &[isize]) {
        for dir in path.into_iter().rev() {
            self.back(*dir);
        }
    }
    fn explore(&mut self, loc: Loc) -> Vec<Loc> {
        let path = self.find_path(loc);
        self.follow_path(&path);
        //println!("Robot Path: {:?}", self.path);
        //println!(" Want Path: {:?}", path);

        let mut queue = Vec::new();
        let mut neighs = Vec::new();
        for neigh in 1..=4 {
            let nloc = self.pos().dir(neigh);
            if self.map.0.contains_key(&nloc) {
                continue;
            }
            let tile = self.peek(neigh);
            if tile == 0 {
                continue;
            }
            neighs.push(neigh);

            let wp = WayPoint {
                loc: nloc,
                neigh: Vec::new(),
                dist: self.dist() + 1,
                oxygen: tile == 2,
                parent_dir: reverse(neigh),
            };

            self.map.0.insert(nloc,wp);
            queue.push(nloc);
        }
        //println!("Robot after explore: {:?}", self.pos());
        self.map.0.get_mut(&self.pos()).unwrap().neigh=neighs;
        self.follow_path_rev(&path);

        queue
    }
    fn dist(&self) -> usize {
        self.path.len()
    }
    fn pos(&self) -> Loc {
        *self.path.last().unwrap_or(&Loc::default())
    }
    fn print_map(&self) {
        let minx = self.map.0.keys().map(|l| l.0).min().unwrap();
        let maxx = self.map.0.keys().map(|l| l.0).max().unwrap() + 1;
        let miny = self.map.0.keys().map(|l| l.1).min().unwrap();
        let maxy = self.map.0.keys().map(|l| l.1).max().unwrap() + 1;

        let mut m = Vec::new();
        for _ in miny..maxy {
            m.push(vec![' '; (maxx-minx) as usize])
        }

        for (k, wp) in &self.map.0 {
            m[(k.1 - miny) as usize][(k.0 - minx) as usize] = if wp.oxygen { 'O' } else { '#' };
        }

        for l in m {
            println!("{}", l.iter().collect::<String>());
        }
    }
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let mut r = Robot::from_file("input")?;
    let mut queue = VecDeque::new();
    queue.push_back(Loc::default());
    while let Some(l) = queue.pop_front() {
        let t = &r.map.0[&l];
        if t.oxygen {
            return Ok(t.dist);
        }
        queue.extend(r.explore(l));
        //println!("{:#?}", r.map);
    }
    Err("No oxygen found!".into())
}

fn flood(mut m: HashSet<Loc>, start: Loc) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut dist = 0;
    while let Some(l) = queue.pop_front() {
        dist = l.1;
        m.take(&l.0);
        for neigh in (1..=4).into_iter().map(|n| l.0.dir(n)) {
            if m.contains(&neigh) {
                queue.push_back((neigh, l.1 + 1));
            }
        }
    }
    dist
}

fn star2() -> Result<usize, Box<dyn Error>> {
    let mut r = Robot::from_file("input")?;
    let mut queue = VecDeque::new();
    queue.push_back(Loc::default());
    while let Some(l) = queue.pop_front() {
        queue.extend(r.explore(l));
    }

    let oxyloc = r.map.0.values().find(|wp| wp.oxygen).unwrap().loc;
    
    r.print_map();

    Ok(flood(r.map.0.keys().cloned().collect(), oxyloc))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
