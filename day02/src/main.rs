use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut forward_flag = false;
    let mut down_flag = false;
    let mut up_flag = false;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(action) = line {
                let movement = action.split_whitespace();
                for part in movement {
                    match part {
                        "forward" => forward_flag = true,
                        "up" => up_flag = true,
                        "down" => down_flag = true,
                        _ => 
                            if forward_flag == true {
                                let change = part.parse::<i32>().unwrap();
                                horizontal += change;
                                forward_flag = false;
                            }
                            else if up_flag == true {
                                let change = part.parse::<i32>().unwrap();
                                depth -= change;
                                up_flag = false;
                            }
                            else if down_flag == true {
                                let change = part.parse::<i32>().unwrap();
                                depth += change;
                                down_flag = false;
                            },
                    }
                }
            }
        }
    }

    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("total distance: {}", depth*horizontal);
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
