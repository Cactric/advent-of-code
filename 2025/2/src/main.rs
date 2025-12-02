use std::fs::read_to_string;
use std::env;
use std::process::exit;

struct Range {
    minimum: u64,
    maximum: u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let puzzleinput = read_to_string(filename).expect("Couldn't read input file");
    let text_ranges: Vec<&str> = puzzleinput.trim_end().split(",").collect();
    
    let mut ranges: Vec<Range> = vec!();
    
    // Try to parse the text_ranges
    for tr in text_ranges {
        let (tmin, tmax) = tr.split_once("-").expect("Invalid range");
        match tmin.parse::<u64>() {
            Ok(imin) => {
                match tmax.parse::<u64>() {
                    Ok(imax) => {
                        ranges.push(Range{minimum: imin, maximum: imax});
                    },
                    Err(_) => {
                        eprintln!("Invalid range maximum");
                        exit(1);
                    }
                }
            },
            Err(_) => {
                eprintln!("Invalid range minimum");
                exit(1);
            },
        }
    }
    
    // Debug: print ranges
    for (i,r) in ranges.iter().enumerate() {
        println!("{}: {} to {}", i, r.minimum, r.maximum);
    }
    
    let mut sum_invalid = 0;
    // Loop through all the ranges
    for r in ranges {
        for id in r.minimum..=r.maximum {
            // Determine if the ID is valid
            
            let idstr = id.to_string();
            if idstr.len() % 2 != 0 {
                // ID is an odd number of digits, it must be valid
                continue;
            } else {
                // Split it in half and check if the digits are the same
                let (left, right) = idstr.split_at(idstr.len() / 2);
                //println!("{} / {}", left, right);
                if left == right {
                    // Invalid!
                    sum_invalid += id;
                    println!("Found invalid id: {}", idstr);
                }
            }
        }
    }
    
    println!("Sum of invalid IDs: {}", sum_invalid);
}
