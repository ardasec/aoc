// Solution to Advent of Code Day 1
// 2025-12-20


use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let message = fs::read_to_string("input_day1")?;

    let mut total: i64 = 50;
    let mut hits_zero = 0;

    for line in message.lines() {
        let (sign, number) = line.split_at(1);
        let value: i64   = number.parse()?;
       // spin the dial 
        let delta = match sign {
            "R" => value,
            "L" => -value,
            _ => 0,
        };

        total += delta;

        // Check if the dial is at number 0 or a multiple of 100
        // as the dial wraps around multiples of 100

        if total % 100 == 0 {
            hits_zero += 1;
        }
    }


println!("The total is: {}", total);
println!("The correct password is: {}", hits_zero);

Ok(())

}
