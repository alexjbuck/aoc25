use std::ops::{Add, Sub};

/// # Panics
/// This function panics if the input cannot be read correctly
/// # Errors
/// This function returns an `AoCError` with variants "Cache" and "Network"
pub fn main() {
    libaoc::evaluate(part_1, part_2, 2025, 1);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ZeroClickingDial {
    dial: Dial,
    clicks: i32,
}

impl From<Dial> for ZeroClickingDial {
    fn from(dial: Dial) -> Self {
        ZeroClickingDial { dial, clicks: 0 }
    }
}

impl Add<i32> for ZeroClickingDial {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let dir = rhs.signum();
        let clicks = (rhs * dir + (self.dial.0 * dir).rem_euclid(100)).div_euclid(100);
        let dial = self.dial + rhs;
        Self { dial, clicks }
    }
}

impl Sub<i32> for ZeroClickingDial {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self.add(-rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Dial(i32);

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl Add<i32> for Dial {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        assert!(self.0 < 100);
        let val = (self.0 + rhs).rem_euclid(100);
        assert!(val < 100);
        Dial(val)
    }
}

fn parse_line(line: &str) -> i32 {
    let sign = match line.chars().next() {
        Some('L') => -1,
        Some('R') => 1,
        Some(_) | None => unreachable!(),
    };
    line[1..].parse::<i32>().unwrap() * sign
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .scan(Dial::default(), |dial, rot| {
            *dial = *dial + rot;
            Some(*dial)
        })
        .filter(|r| *r == Dial(0))
        .collect::<Vec<_>>()
        .len()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .inspect(|r| println!("Rotating {r}"))
        .scan(ZeroClickingDial::default(), |dial, rot| {
            *dial = *dial + rot;
            Some(*dial)
        })
        .inspect(|d| println!("{d:?}"))
        .map(|d| d.clicks)
        .inspect(|c| println!("count {c}"))
        .sum::<i32>()
        .try_into()
        .expect("Expected positive value")
}
const _: () = assert!(usize::BITS >= 32, "usize must be at least 32 bits");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clicking_dial() {
        let dial = ZeroClickingDial::from(Dial(10)) + 10;
        assert!(dial.clicks == 0);
        assert!(dial.dial.0 == 20);
        let dial = ZeroClickingDial::from(Dial(0)) + 10;
        assert!(dial.clicks == 0);
        assert!(dial.dial.0 == 10);
        let dial = ZeroClickingDial::from(Dial(0)) + 100;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(0)) - 100;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(0)) - 110;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 90);
        let dial = ZeroClickingDial::from(Dial(0)) + 110;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 10);
        let dial = ZeroClickingDial::from(Dial(50)) - 50;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(50)) + 50;
        assert!(dial.clicks == 1);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(50)) + 150;
        assert!(dial.clicks == 2);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(50)) - 150;
        assert!(dial.clicks == 2);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(0)) - 1000;
        assert!(dial.clicks == 10);
        assert!(dial.dial.0 == 0);
        let dial = ZeroClickingDial::from(Dial(0)) + 1050;
        assert!(dial.clicks == 10);
        assert!(dial.dial.0 == 50);
    }
}
