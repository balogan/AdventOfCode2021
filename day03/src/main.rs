use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pos0 = 0;
    let mut pos1 = 0;
    let mut pos2 = 0;
    let mut pos3 = 0;
    let mut pos4 = 0;
    let mut pos5 = 0;
    let mut pos6 = 0;
    let mut pos7 = 0;
    let mut pos8 = 0;
    let mut pos9 = 0;
    let mut pos10 = 0;
    let mut pos11 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(action) = line {
                let code = action;
                if code.chars().nth(0).unwrap() == '1' {
                    pos0 += 1
                } else {
                    pos0 -= 1
                }
                if code.chars().nth(1).unwrap() == '1' {
                    pos1 += 1
                } else {
                    pos1 -= 1
                }
                if code.chars().nth(2).unwrap() == '1' {
                    pos2 += 1
                } else {
                    pos2 -= 1
                }
                if code.chars().nth(3).unwrap() == '1' {
                    pos3 += 1
                } else {
                    pos3 -= 1
                }
                if code.chars().nth(4).unwrap() == '1' {
                    pos4 += 1
                } else {
                    pos4 -= 1
                }
                if code.chars().nth(5).unwrap() == '1' {
                    pos5 += 1
                } else {
                    pos5 -= 1
                }
                if code.chars().nth(6).unwrap() == '1' {
                    pos6 += 1
                } else {
                    pos6 -= 1
                }
                if code.chars().nth(7).unwrap() == '1' {
                    pos7 += 1
                } else {
                    pos7 -= 1
                }
                if code.chars().nth(8).unwrap() == '1' {
                    pos8 += 1
                } else {
                    pos8 -= 1
                }
                if code.chars().nth(9).unwrap() == '1' {
                    pos9 += 1
                } else {
                    pos9 -= 1
                }
                if code.chars().nth(10).unwrap() == '1' {
                    pos10 += 1
                } else {
                    pos10 -= 1
                }
                if code.chars().nth(11).unwrap() == '1' {
                    pos11 += 1
                } else {
                    pos11 -= 1
                }
            }
        }
    }

    let gamma_rate = build_gamma_rate(
        pos0, pos1, pos2, pos3, pos4, pos5, pos6, pos7, pos8, pos9, pos10, pos11,
    );
    let epsilon_rate = build_epsilon_rate(
        pos0, pos1, pos2, pos3, pos4, pos5, pos6, pos7, pos8, pos9, pos10, pos11,
    );
    println!("gamma: {}, epsilon: {}", gamma_rate, epsilon_rate);
    let power_consumption = gamma_rate * epsilon_rate;
    println!("power: {}", power_consumption);
}

fn build_gamma_rate(
    pos0: i32,
    pos1: i32,
    pos2: i32,
    pos3: i32,
    pos4: i32,
    pos5: i32,
    pos6: i32,
    pos7: i32,
    pos8: i32,
    pos9: i32,
    pos10: i32,
    pos11: i32,
) -> i32 {
    let char0;
    let char1;
    let char2;
    let char3;
    let char4;
    let char5;
    let char6;
    let char7;
    let char8;
    let char9;
    let char10;
    let char11;

    if pos0 > 0 {
        char0 = "1";
    } else {
        char0 = "0";
    }
    if pos1 > 0 {
        char1 = "1";
    } else {
        char1 = "0";
    }
    if pos2 > 0 {
        char2 = "1";
    } else {
        char2 = "0";
    }
    if pos3 > 0 {
        char3 = "1";
    } else {
        char3 = "0";
    }
    if pos4 > 0 {
        char4 = "1";
    } else {
        char4 = "0";
    }
    if pos5 > 0 {
        char5 = "1";
    } else {
        char5 = "0";
    }
    if pos6 > 0 {
        char6 = "1";
    } else {
        char6 = "0";
    }
    if pos7 > 0 {
        char7 = "1";
    } else {
        char7 = "0";
    }
    if pos8 > 0 {
        char8 = "1";
    } else {
        char8 = "0";
    }
    if pos9 > 0 {
        char9 = "1";
    } else {
        char9 = "0";
    }
    if pos10 > 0 {
        char10 = "1";
    } else {
        char10 = "0";
    }
    if pos11 > 0 {
        char11 = "1";
    } else {
        char11 = "0";
    }

    let binary = String::from(char0)
        + char1
        + char2
        + char3
        + char4
        + char5
        + char6
        + char7
        + char8
        + char9
        + char10
        + char11;
    let result = i32::from_str_radix(&binary, 2).unwrap();
    result
}

fn build_epsilon_rate(
    pos0: i32,
    pos1: i32,
    pos2: i32,
    pos3: i32,
    pos4: i32,
    pos5: i32,
    pos6: i32,
    pos7: i32,
    pos8: i32,
    pos9: i32,
    pos10: i32,
    pos11: i32,
) -> i32 {
    let char0;
    let char1;
    let char2;
    let char3;
    let char4;
    let char5;
    let char6;
    let char7;
    let char8;
    let char9;
    let char10;
    let char11;

    if pos0 < 0 {
        char0 = "1";
    } else {
        char0 = "0";
    }
    if pos1 < 0 {
        char1 = "1";
    } else {
        char1 = "0";
    }
    if pos2 < 0 {
        char2 = "1";
    } else {
        char2 = "0";
    }
    if pos3 < 0 {
        char3 = "1";
    } else {
        char3 = "0";
    }
    if pos4 < 0 {
        char4 = "1";
    } else {
        char4 = "0";
    }
    if pos5 < 0 {
        char5 = "1";
    } else {
        char5 = "0";
    }
    if pos6 < 0 {
        char6 = "1";
    } else {
        char6 = "0";
    }
    if pos7 < 0 {
        char7 = "1";
    } else {
        char7 = "0";
    }
    if pos8 < 0 {
        char8 = "1";
    } else {
        char8 = "0";
    }
    if pos9 < 0 {
        char9 = "1";
    } else {
        char9 = "0";
    }
    if pos10 < 0 {
        char10 = "1";
    } else {
        char10 = "0";
    }
    if pos11 < 0 {
        char11 = "1";
    } else {
        char11 = "0";
    }

    let binary = String::from(char0)
        + char1
        + char2
        + char3
        + char4
        + char5
        + char6
        + char7
        + char8
        + char9
        + char10
        + char11;
    let result = i32::from_str_radix(&binary, 2).unwrap();
    result
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
