use std::error::Error;
use aoc2019::{lines,intcode::Intcode};

fn star1() -> Result<isize, Box<Error>> {
    let mut ic = lines("input")?.next().unwrap().parse::<Intcode>()?;
    ic.run_input(1)?.last().cloned().ok_or(Box::<dyn Error>::from("no output".to_owned()))
}
fn star2() -> Result<isize, Box<Error>>{
    let mut ic = lines("input")?.next().unwrap().parse::<Intcode>()?;
    ic.run_input(5)?.last().cloned().ok_or(Box::<dyn Error>::from("no output".to_owned()))
}

fn main() -> Result<(),Box<Error>>{
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use aoc2019::intcode::Intcode;
    use std::error::Error;
    use std::str::FromStr;
    #[test]
    fn examples() -> Result<(), Box<Error>> {
        assert_eq!(9999, *Intcode::from_str("3,0,4,0,99")?.run_input(9999)?.last().unwrap());

        assert_eq!(0, *Intcode::from_str("3,9,8,9,10,9,4,9,99,-1,8")?.run_input(1)?.last().unwrap());
        assert_eq!(1, *Intcode::from_str("3,9,8,9,10,9,4,9,99,-1,8")?.run_input(8)?.last().unwrap());

        assert_eq!(1, *Intcode::from_str("3,9,7,9,10,9,4,9,99,-1,8")?.run_input(1)?.last().unwrap());
        assert_eq!(0, *Intcode::from_str("3,9,7,9,10,9,4,9,99,-1,8")?.run_input(8)?.last().unwrap());

        assert_eq!(0, *Intcode::from_str("3,3,1108,-1,8,3,4,3,99")?.run_input(9)?.last().unwrap());
        assert_eq!(1, *Intcode::from_str("3,3,1108,-1,8,3,4,3,99")?.run_input(8)?.last().unwrap());

        assert_eq!(0, *Intcode::from_str("3,3,1107,-1,8,3,4,3,99")?.run_input(8)?.last().unwrap());
        assert_eq!(1, *Intcode::from_str("3,3,1107,-1,8,3,4,3,99")?.run_input(0)?.last().unwrap());

        assert_eq!(0, *Intcode::from_str("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")?.run_input(0)?.last().unwrap());
        assert_eq!(1, *Intcode::from_str("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")?.run_input(-1)?.last().unwrap());

        assert_eq!(0, *Intcode::from_str("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")?.run_input(0)?.last().unwrap());
        assert_eq!(1, *Intcode::from_str("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")?.run_input(-1)?.last().unwrap());

        assert_eq!(999, *Intcode::from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")?.run_input(7)?.last().unwrap());
        assert_eq!(1000, *Intcode::from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")?.run_input(8)?.last().unwrap());
        assert_eq!(1001, *Intcode::from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")?.run_input(9)?.last().unwrap());
        Ok(())
    }
}
