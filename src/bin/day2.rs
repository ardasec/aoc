use std::fs;
use std::io::{self, Read};

fn is_invalid(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    &s[..half] == &s[half..]
}

fn main() -> io::Result<()> {
    let mut content = String::new();
    let mut file = fs::File::open("input_day2")?;
    file.read_to_string(&mut content)?;

    // cleanup output
    let content = content.replace('\n', "").replace(' ', "");
    
    let mut total_sum: u64 = 0;
    
    for range in content.split(',') {
        if range.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let start: u64 = parts[0].parse().expect("Invalid start number");
        let end: u64 = parts[1].parse().expect("Invalid end number");
        
        for n in start..=end {
            if is_invalid(n) {
                println!("Found invalid ID: {}", n);
                total_sum += n;
            }
        }
    }
    
    println!("\nTotal Sum: {}", total_sum);
    
    Ok(())
}
