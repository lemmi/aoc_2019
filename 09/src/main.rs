use aoc2019::lines;
use aoc2019::intcode::Intcode;
use std::str::FromStr;
use std::error::Error;

fn star1() -> Result<isize, Box<Error>> {
    let prog = lines("input")?.next().unwrap();
    let out = Intcode::from_str(&prog)?.run_input(&[1])?;
    Ok(out[0])
}
fn star2() -> Result<isize, Box<Error>> {
    let prog = lines("input")?.next().unwrap();
    let out = Intcode::from_str(&prog)?.run_input(&[2])?;
    Ok(out[0])
}

fn main() -> Result<(), Box<Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn relquine() -> Result<(),Box<Error>> {
        assert_eq!(
            vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
            Intcode::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")?.run_input(&[])?);
        Ok(())
    }
    #[test]
    fn relmul() -> Result<(),Box<Error>> {
        assert_eq!(
            1219070632396864,
            *Intcode::from_str("1102,34915192,34915192,7,4,7,99,0")?.run_input(&[])?.last().unwrap());
        Ok(())
    }
    #[test]
    fn rellarge() -> Result<(),Box<Error>> {
        assert_eq!(
            1125899906842624,
            *Intcode::from_str("104,1125899906842624,99")?.run_input(&[])?.last().unwrap());
        Ok(())
    }
}