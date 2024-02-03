use std::env::args;
use std::io::*;
struct BreedCalculation;

impl BreedCalculation {
    fn pig() -> Result<()> {
        let argc:   Vec<String> = args().collect();
        let gens:   i32 = argc[1].parse().unwrap();
        let mut num:    f32 = 2.;
        let mut numa:   f32 = 0.;
        println!("|gen|     pig     |       carrots     |");
        for i in 0..gens+1 {
            if i >= 1 {num = num + num/2.};
            println!("|{} |{}           |{}                 |", i, num.round(), numa);
            numa = num.round();
        }
        Ok(())
    }
}

fn main() {
    // let argc: Vec<String> = args().collect();
    let _pig = BreedCalculation::pig();
}
