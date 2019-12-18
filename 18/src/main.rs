use aoc2019::lines;
use std::error::Error;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

type Key = u8;

#[derive(Eq,PartialEq,Debug,Clone)]
enum Tile {
    Wall,
    Hall,
    Robot,
    Key(Key),
    Door(Key),
}

impl Tile {
    fn is_free(&self, keys: KeySet) -> bool {
        match self {
            Self::Wall => false,
            Self::Door(x) if !keys.contains(*x) => false,
            _ => true,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Hall,
            '@' => Self::Robot,
            x if x.is_ascii_lowercase() => Self::Key(x as u8 - 'a' as u8),
            x if x.is_ascii_uppercase() => Self::Door(x as u8 - 'A' as u8),
            x => panic!("unkown tile {}",x),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Wall => String::from("██"),
            Self::Hall => String::from("  "),
            Self::Robot => String::from("@@"),
            Self::Key(k) => { let c = k + 'a' as u8; String::from_utf8(vec![c,c]).unwrap() }
            Self::Door(d) => { let c = d + 'A' as u8; String::from_utf8(vec![c,c]).unwrap() }
        })
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,Default)]
struct KeySet(u32);
impl KeySet {
    fn insert(&self, k: Key) -> KeySet {
        KeySet(self.0 | 1<<k)
    }
    fn contains(&self, k: Key) -> bool {
        (self.0 & 1<<k) > 0
    }
}
impl std::fmt::Display for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for s in 0..32 {
            if self.0 & 1 << s > 0 {
                let c = (s + 'a' as u8) as char;
                write!(f, "{}", c)?;
            }
        }
        write!(f, "")
    }
}

fn fmt_line(l: &[Tile]) -> String {
    l.iter().map(|t| t.to_string()).collect()
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,PartialOrd,Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighs(&self, m: &Map, k: KeySet) -> Vec<Pos> {
        [
            Pos{ x: self.x - 1, y: self.y, },
            Pos{ x: self.x + 1, y: self.y, },
            Pos{ x: self.x, y: self.y - 1, },
            Pos{ x: self.x, y: self.y + 1, },
        ].iter().filter(|p| m[p.y][p.x].is_free(k)).cloned().collect()
    }
}

type Map = Vec<Vec<Tile>>;
fn map_from_file(f: &str) -> Result<Map,Box<dyn Error>> {
    Ok(lines(f)?.map(|l| l.chars().map(|c| Tile::from(c)).collect::<Vec<Tile>>()).collect::<Map>())
}

fn find_bot(m: &Map) -> Pos {
    for (y,l) in m.iter().enumerate() {
        if let Some(x) = l.iter().position(|t| *t == Tile::Robot) {
            return Pos{x,y}
        }
    }
    panic!("no bot found!")
}

fn search(m: &Map) -> usize {
    let r = find_bot(m);
    let goal = all_keys(m);
    bfs(m, r, KeySet::default(), goal, &mut HashMap::new())
} 
fn bfs(m: &Map, from: Pos, keys: KeySet, goal: KeySet, cache: &mut HashMap<(Pos,KeySet),usize>) -> usize {
    if keys == goal {
        return 0
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((from,0));


    let mut ret = std::usize::MAX;
    while let Some((p,d)) = queue.pop_front() {
        visited.insert(p);
        if let Tile::Key(k) = m[p.y][p.x] {
            if !keys.contains(k) {
                let nkeys = keys.insert(k);
                let s = match cache.get(&(p,nkeys)) {
                    Some(s) => *s,
                    None => {
                        let s = bfs(m, p, keys.insert(k), goal, cache);
                        cache.insert((p,nkeys),s);
                        s
                    }
                };
                ret = ret.min(d+s);
                continue
            }
        }

        for neigh in p.neighs(m, keys) {
            if visited.contains(&neigh) {
                continue;
            }
            queue.push_back((neigh, d+1));
        }
    }
    ret
}

fn qsearch(m: &Map) -> usize {
    let mut m = m.clone();
    let r = find_bot(&m);
    let goal = all_keys(&m);

    m[r.y][r.x] = Tile::Wall;
    m[r.y-1][r.x] = Tile::Wall;
    m[r.y+1][r.x] = Tile::Wall;
    m[r.y][r.x-1] = Tile::Wall;
    m[r.y][r.x+1] = Tile::Wall;

    let r = vec![
        Pos{x: r.x - 1, y: r.y - 1},
        Pos{x: r.x - 1, y: r.y + 1},
        Pos{x: r.x + 1, y: r.y - 1},
        Pos{x: r.x + 1, y: r.y + 1},
    ];

    if cfg!(test) {
        debug_state(&m, &r, &HashSet::new(), KeySet::default(), 0 );
    }
    qbfs(&m, r, KeySet::default(), goal, &mut HashMap::new())
} 
fn qbfs(m: &Map, mut froms: Vec<Pos>, keys: KeySet, goal: KeySet, cache: &mut HashMap<(Vec<Pos>, KeySet), usize>) -> usize {
    if keys == goal {
        return 0
    }

    let mut ret = std::usize::MAX;
    for _ in 0..froms.len() {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((froms[0], 0));
        while let Some((p,d)) = queue.pop_front() {
            //debug_state(m, &froms, &visited, keys, d );
            //println!("{:?}", froms);
            visited.insert(p);

            if let Tile::Key(k) = m[p.y][p.x] {
                if !keys.contains(k) {
                    let nkeys = keys.insert(k);
                    let mut nfroms = vec![p, froms[1], froms[2], froms[3]];
                    nfroms.sort();
                    let s = match cache.get(&(nfroms.clone(),nkeys)) {
                        Some(s) => *s,
                        None => {
                            let s = qbfs(m, nfroms.clone(), keys.insert(k), goal, cache);
                            cache.insert((nfroms,nkeys),s);
                            s
                        }
                    };
                    ret = ret.min(d+s);
                    continue
                }
            }

            for neigh in p.neighs(m, keys) {
                if visited.contains(&neigh) {
                    continue;
                }
                queue.push_back((neigh, d+1));
            }

        }

        froms.rotate_left(1);
    }
    ret
}

fn print_state(m: &Map) {
    for l in m {
        println!("{}", fmt_line(&l));
    }
}

fn debug_state(m: &Map, r: &[Pos], v: &HashSet<Pos>, keys: KeySet, d: usize) {
    for (y, l) in m.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            let p = Pos{x,y};

            if r.iter().any(|r| *r == p) {
                print!("{}",Tile::Robot);
            } else if v.contains(&p) {
                print!("░░");
            } else if *t == Tile::Robot {
                print!("{}",Tile::Hall);
            } else {
                print!("{}",t)
            }
        }
        println!();
    }

    println!("keys: {}", keys);
    println!("steps: {}", d);
    println!();
    std::thread::sleep(std::time::Duration::from_millis(10));
}
    
fn all_keys(m: &Map) -> KeySet {
    m.iter().flat_map(|l| l.iter()).fold(KeySet::default(), |set, t| if let Tile::Key(k) = t { set.insert(*k) } else { set })
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let m = map_from_file("input")?;
    //print_state(&m);
    Ok(search(&m))
}
fn star2() -> Result<usize, Box<dyn Error>> {
    let m = map_from_file("input")?;
    Ok(qsearch(&m))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<(), Box<dyn Error>>{
        let m = map_from_file("test1")?;
        print_state(&m);
        assert_eq!(132, search(&m));
        Ok(())
    }

    #[test]
    fn t2() -> Result<(), Box<dyn Error>>{
        let m = map_from_file("test2")?;
        print_state(&m);
        assert_eq!(136, search(&m));
        Ok(())
    }

    #[test]
    fn t3() -> Result<(), Box<dyn Error>>{
        let m = map_from_file("test3")?;
        print_state(&m);
        assert_eq!(86, search(&m));
        Ok(())
    }

    #[test]
    fn t4() -> Result<(), Box<dyn Error>>{
        let m = map_from_file("test4")?;
        print_state(&m);
        assert_eq!(81, search(&m));
        Ok(())
    }

    #[test]
    fn qt1() -> Result<(), Box<dyn Error>>{
        let m = map_from_file("qtest1")?;
        assert_eq!(8, qsearch(&m));
        Ok(())
    }
}
