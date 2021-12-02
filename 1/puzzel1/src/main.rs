use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;

    let lines = BufReader::new(file).lines();

    let mut prev_depth = 0;
    let mut increase_count = 0;
    for (index, line) in lines.enumerate() {
        if let Ok(depth_str) = line {
            let depth = depth_str.parse::<i16>()?;

            if index != 0 && depth > prev_depth {
                increase_count += 1;
            }

            prev_depth = depth;
        }
    }

    println!("depth increased {} times.", increase_count);

    Ok(())
}
