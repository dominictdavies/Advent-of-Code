use core::panic;
use std::fmt;
use std::fs;

struct Dial {
    pointing_at: u8,
    num_zeros: u16,
}

impl Dial {
    fn new(pointing_at: u8) -> Self {
        Self {
            pointing_at,
            num_zeros: 0,
        }
    }

    fn get_num_zeros(&self) -> u16 {
        self.num_zeros
    }

    fn wrapping_sub(&mut self, rhs: u16) {
        self.num_zeros += rhs / 100;
        let rhs = (rhs % 100) as u8;

        let result = (self.pointing_at + 100) - rhs;
        if result <= 100 && self.pointing_at > 0 {
            self.num_zeros += 1;
        }

        self.pointing_at = result % 100;
    }

    fn wrapping_add(&mut self, rhs: u16) {
        self.num_zeros += rhs / 100;
        let rhs = (rhs % 100) as u8;

        let result = self.pointing_at + rhs;
        if result >= 100 {
            self.num_zeros += 1;
        }

        self.pointing_at = result % 100;
    }
}

impl PartialEq<u8> for Dial {
    fn eq(&self, other: &u8) -> bool {
        self.pointing_at == *other
    }
}

impl fmt::Display for Dial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pointing_at)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut dial = Dial::new(50);

    for line in lines {
        if line == "" {
            continue;
        }

        let (direction, distance) = line.split_at(1);
        let distance: u16 = distance.parse().expect("Failed to read the distance");

        match direction {
            "L" => dial.wrapping_sub(distance),
            "R" => dial.wrapping_add(distance),
            _ => panic!("Direction was not L or R"),
        }
    }

    let num_zeros = dial.get_num_zeros();
    println!("num_zeros: {num_zeros}");
}
