use std::fs;
use std::io::{self, Read};

fn solve_line(line: &str) -> u64 {
    let digits: Vec<u32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let k = 12;
    let n = digits.len();

    if n < k {
        return 0;
    }

    let mut stack: Vec<u32> = Vec::with_capacity(k);

    for (i, &digit) in digits.iter().enumerate() {
        while let Some(&last) = stack.last() {
            if last < digit && (stack.len() - 1 + (n - i) >= k) {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k {
            stack.push(digit);
        }
    }

    let mut res_val: u64 = 0;
    for &d in &stack {
        res_val = res_val * 10 + d as u64;
    }

    res_val
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
