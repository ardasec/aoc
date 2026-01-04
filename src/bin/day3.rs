use std::fs;
use std::io::{self, Read};

fn solve_line(line: &str) -> u64 {
    let digits: Vec<u32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() < 2 {
        return 0;
    }

    let mut max_res: u64 = 0;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let val = (digits[i] * 10 + digits[j]) as u64;
            if val > max_res {
                max_res = val;
            }
        }
    }

    max_res
}

fn main() -> io::Result<()> {
    let mut content = String::new();
    let mut file = fs::File::open("input_day3")?;
    file.read_to_string(&mut content)?;

    let mut total_sum: u64 = 0;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let val = solve_line(line);
        total_sum += val;
    }
    
    println!("Total Sum: {}", total_sum);
    
    Ok(())
}
