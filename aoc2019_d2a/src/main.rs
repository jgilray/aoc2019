// AoC 2019 day 2

// custom error type
#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../intcode.dat");
    const TARGET: usize = 19690720;

    let mut ic: Vec<usize> = input
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let ic_clone = ic.clone();
    let mut ip = 0;

    while ip < ic.len() && ic[ip] != 99 {
        match ic[ip] {
            1 => {
                let loc = ic[ip + 3];
                ic[loc] = ic[ic[ip + 1]] + ic[ic[ip + 2]];
                ip += 4;
            }
            2 => {
                let loc = ic[ip + 3];
                ic[loc] = ic[ic[ip + 1]] * ic[ic[ip + 2]];
                ip += 4;
            }
            _ => {
                return Err(Box::new(Error::new(&format!(
                    "bad opcode in input: {}",
                    ic[ip]
                ))))
            }
        }
    }

    // part 1
    println!("aoc2a: {}", ic[0]);

    // part 2
    'p2: for noun in 0..=99 {
        for verb in 0..=99 {
            ic = ic_clone.clone();
            ic[1] = noun;
            ic[2] = verb;
            ip = 0;
            while ic[ip] != 99 {
                match ic[ip] {
                    1 => {
                        let loc = ic[ip + 3];
                        ic[loc] = ic[ic[ip + 1]] + ic[ic[ip + 2]];
                        ip += 4;
                    }
                    2 => {
                        let loc = ic[ip + 3];
                        ic[loc] = ic[ic[ip + 1]] * ic[ic[ip + 2]];
                        ip += 4;
                    }
                    _ => {
                        return Err(Box::new(Error::new(&format!(
                            "bad opcode in input: {}",
                            ic[ip]
                        ))))
                    }
                }
            }
            if ic[0] == TARGET {
                println!("aoc2b: {}", 100 * noun + verb);
                break 'p2;
            }
        }
    }

    Ok(())
}
