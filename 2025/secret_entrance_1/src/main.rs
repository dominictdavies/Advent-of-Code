use core::panic;
use std::fmt;
use std::fs;

const DIAL_START: u8 = 50;

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

    fn get_num_zeros(self) -> u16 {
        self.num_zeros
    }

    fn wrapping_sub(&mut self, rhs: u16) {
        let rhs: u8 = (rhs % 100) as u8;

        if self.pointing_at.checked_sub(rhs).is_none() {
            self.pointing_at = (self.pointing_at + 100) - rhs;
        } else {
            self.pointing_at = self.pointing_at - rhs;
        }

        if self.pointing_at == 0 {
            self.num_zeros += 1;
        }
    }

    fn wrapping_add(&mut self, rhs: u16) {
        let rhs: u8 = (rhs % 100) as u8;

        self.pointing_at = (self.pointing_at + rhs) % 100;

        if self.pointing_at == 0 {
            self.num_zeros += 1;
        }
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

    let mut dial = Dial::new(DIAL_START);

    for line in lines {
        if line == "" {
            continue;
        }

        let (direction, distance) = line.split_at(1);
        let distance: u16 = distance.parse().expect("Failed to read the distance");

        if direction == "L" {
            dial.wrapping_sub(distance);
        } else if direction == "R" {
            dial.wrapping_add(distance);
        } else {
            panic!("Direction was not 'L' or 'R'");
        }
    }

    let num_zeros = dial.get_num_zeros();
    println!("zero_count: {num_zeros}");
}
