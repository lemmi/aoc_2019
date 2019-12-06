use std::error::Error;
use std::collections::HashMap;
use aoc2019::lines;

fn parse_edge(l: String) -> Result<(String,String), Box<Error>> {
    if let Some(p) = l.find(')') {
        Ok((l[p+1..].to_string(),l[..p].to_string()))
    } else {
        Err(Box::<dyn Error>::from("Invalid Edge description".to_string()))
    }
}

fn parents<'a>(e: &'a HashMap<String,String>,k:&str) -> Vec<&'a str> {
    let mut v = k;
    let mut ret = Vec::new();
    while let Some(k) = e.get(v) {
        v = k;
        ret.push(k.as_str());
    }
    ret
}

fn orbits(e: &HashMap<String,String>) -> usize {
    let mut ret = 0;
    for k in e.keys() {
        ret += parents(e,k).len();
    }
    ret
}

fn transfer(e: &HashMap<String,String>, from: &str, to: &str) -> usize {
    let p_you = parents(&e, from);
    let p_san = parents(&e, to);

    let shared = p_you.iter().rev().zip(p_san.iter().rev()).take_while(|(o1,o2)| o1 == o2).count();

    p_you.len() + p_san.len() - 2*shared
}

fn read_edges(src: &str) -> Result<HashMap<String,String>,Box<Error>> {
    lines(src)?.map(parse_edge).collect::<Result<HashMap<_,_>,_>>()
}

fn star1() -> Result<usize,Box<Error>> {
    Ok(orbits(&read_edges("input")?))
}
fn star2() -> Result<usize,Box<Error>> {
    let e = read_edges("input")?;

    Ok(transfer(&e, "YOU", "SAN"))
}

fn main() -> Result<(),Box<Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count_orbits() -> Result<(),Box<Error>>{
        assert_eq!(42,orbits(&read_edges("test")?));
        Ok(())
    }
    #[test]
    fn transfer_orbits() -> Result<(),Box<Error>>{
        assert_eq!(4,transfer(&read_edges("test2")?,"YOU","SAN"));
        Ok(())
    }
}
