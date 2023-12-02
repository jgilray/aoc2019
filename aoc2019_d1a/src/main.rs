// AoC 2019 day 1

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
    let input = include_str!("../../module_masses.dat");
    let mut ansa = 0_isize;
    let mut ansb = 0_isize;

    let lines: Vec<&str> = input.split('\n').collect();

    for l in lines {
        if l.is_empty() {
            continue;
        }

        let val = l.parse::<isize>();

        if val.is_ok() {
            let mut guess = val.unwrap() / 3 - 2;
            ansa += guess;

            let mut sum = 0;
            while guess > 0 {
                sum += guess;
                guess = guess / 3 - 2;
            }
            ansb += sum;
        } else {
            return Err(Box::new(Error::new(&format!("bad line in input: {}", l))));
        }
    }

    // part 1
    println!("aoc1a: {}", ansa);

    // part 2
    println!("aoc1b: {}", ansb);

    Ok(())
}
