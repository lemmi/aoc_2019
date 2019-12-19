use aoc2019::intcode::Intcode;
use std::error::Error;

fn star1() -> Result<usize, Box<dyn Error>> {
    let drone = Intcode::from_file("input")?;

    let mut affected = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut ic = drone.clone();
            if ic.run_input(&[x, y]).out()[0] == 1 {
                //print!("#");
                affected += 1;
            } else {
                //print!(" ");
            }
        }
        //println!();
    }
    Ok(affected)
}

fn find_border(drone: &Intcode, y: isize, guessx: isize, guessw: isize) -> (isize, isize) {
    let mut width = 0;
    let mut skip = guessx;
    for x in guessx..=2 * y {
        let c = drone.clone().run_input(&[x, y]).out()[0];
        match c {
            0 => skip += 1,
            1 => {
                width = 1;
                break;
            }
            _ => panic!("wat?"),
        }
    }

    if width == 0 {
        return (0, 0);
    }

    width = 0.max(guessw - 1);

    let mut step = 1;
    loop {
        let c = drone.clone().run_input(&[skip + width + step - 1, y]).out()[0];
        match (c, step) {
            (0, 1) => break,
            (1, _) => {
                width += step;
                step *= 2
            }
            (0, s) if s > 1 => {
                step /= 2;
            }
            (_, _) => panic!("wat?"),
        }
    }

    (skip, width)
}

fn test_fit(
    drone: &Intcode,
    y: isize,
    width: isize,
    guessx: isize,
    guessw: isize,
) -> (bool, isize, isize) {
    let (upskip, upwidth) = find_border(drone, y, guessx, guessw);
    if upwidth < width {
        return (false, upskip, upwidth);
    }
    let (downskip, _) = find_border(drone, y + width - 1, upskip, upwidth);

    (downskip <= upskip + upwidth - width, upskip, upwidth)
}

fn star2() -> Result<isize, Box<dyn Error>> {
    let drone = Intcode::from_file("input")?;

    let mut y = 0;
    let mut guessx = 0;
    let mut guessw = 0;
    loop {
        let (fit, x, w) = test_fit(&drone, y, 100, guessx, guessw);
        //println!("{} {}", y, fit);
        if fit {
            break;
        }
        y += 1;
        guessx = x;
        guessw = w;
    }

    let (upskip, upwidth) = find_border(&drone, y, 0, 0);

    let x = upskip + upwidth - 100;

    //for yy in y..y+100 {
    //    let (skip, width) = find_border(&drone, yy, upskip,0);

    //    let empty = skip - upskip;
    //    let left  = upskip + upwidth - 100 - upskip;
    //    let right = upskip + upwidth - upskip;
    //    let end   = skip + width - upskip;

    //    for _ in 0..empty {
    //        print!(" ");
    //    }
    //    for _ in empty..left {
    //        print!("#");
    //    }
    //    for _ in left..right{
    //        print!("O");
    //    }
    //    for _ in right..end {
    //        print!("#");
    //    }
    //    println!();
    //}
    Ok(10_000 * x + y)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Star 01: {}", star1()?);
    println!("Star 02: {}", star2()?);
    Ok(())
}
