use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut increased_count = 0;
        let mut last_depth = 0;
        for line in lines {
            if let Ok(value) = line {
                let depth = value.parse::<i32>().unwrap();
                println!(
                    "depth: {}, last_depth: {}, increased_count: {}",
                    depth, last_depth, increased_count
                );
                if depth > last_depth {
                    last_depth = depth;
                    increased_count += 1;
                } else {
                    last_depth = depth;
                }
            }
        }
        println!("Total increases: {}", increased_count - 1);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
