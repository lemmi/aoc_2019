use aoc2019::lines;
use std::error::Error;
use std::str::FromStr;

struct Image {
    pix: Vec<u8>,
}

impl FromStr for Image {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Image {
            pix: s.chars().map(|c| c as u8 - '0' as u8).collect(),
        })
    }
}

fn star1() -> Result<usize, Box<dyn Error>> {
    let img = lines("input")?.next().unwrap().parse::<Image>()?;
    let ls = 25 * 6;

    let layer = img
        .pix
        .chunks(ls)
        .min_by_key(|l| l.into_iter().filter(|&&p| p == 0).count())
        .unwrap();

    let ones = layer.iter().filter(|&&p| p == 1).count();
    let twos = layer.iter().filter(|&&p| p == 2).count();

    Ok(ones * twos)
}

fn merge_layer(upper: Vec<u8>, lower: &[u8]) -> Vec<u8> {
    upper
        .into_iter()
        .zip(lower)
        .map(|(u, &l)| if u == 2 { l } else { u })
        .collect()
}

fn print_layer(l: &[u8]) {
    for line in l.chunks(25) {
        println!(
            "{:?}",
            line.iter()
                .map(|&c| if c == 0 { ' ' } else { '#' })
                .collect::<String>()
        );
    }
}

fn star2() -> Result<usize, Box<dyn Error>> {
    let img = lines("input")?.next().unwrap().parse::<Image>()?;
    let ls = 25 * 6;

    let out = img.pix.chunks(ls).fold(vec![2; ls], merge_layer);
    print_layer(&out);

    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
