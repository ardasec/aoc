// Solution to Advent of Code Day 1 pt. 2
// 2025-12-20


use std::fs;
use std::error::Error;

fn floor_div(a: i64, b: i64) -> i64 {
    // helper function to deal with negative numbers
    let mut q = a / b;
    if a % b != 0 && ((a < 0) != (b < 0)) {
        q -= 1;
    }
    q
}



fn main() -> Result<(), Box<dyn Error>> {

    let message = fs::read_to_string("input_day1")?;

    let mut total: i64 = 50;
    let mut crossings: usize = 0;

    for line in message.lines() {
        let (sign, number) = line.split_at(1);
        let value: i64 = number.parse()?;
       // spin the dial 
        let delta = match sign {
            "R" => value,
            "L" => -value,
            _ => 0,
        };

        let next_total = total + delta;
        let low = total.min(next_total);
        let high = total.max(next_total);

        let crossed = floor_div(high, 100) - floor_div(low, 100);
        crossings += crossed.abs() as usize;
        total = next_total;


    }
        



println!("The total is: {}", total);
println!("The amount of times the dial passed 0 is: {}", crossings);

Ok(())

}
