use core::panic;
use std::fmt;
use std::fs;

const DIAL_START: u8 = 50;

struct Dial(u8);

impl Dial {
    fn wrapping_sub(&mut self, rhs: u16) {
        let rhs: u8 = (rhs % 100) as u8;

        if self.0.checked_sub(rhs).is_none() {
            self.0 = (self.0 + 100) - rhs;
        } else {
            self.0 = self.0 - rhs;
        }
    }

    fn wrapping_add(&mut self, rhs: u16) {
        let rhs: u8 = (rhs % 100) as u8;

        self.0 = (self.0 + rhs) % 100;
    }
}

impl PartialEq<u8> for Dial {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for Dial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut dial = Dial(DIAL_START);
    let mut zero_count: u16 = 0;

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

        if dial == 0 {
            zero_count += 1;
        }
    }

    println!("zero_count: {zero_count}");
}
