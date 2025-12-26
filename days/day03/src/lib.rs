fn max_joltage(line: &str) -> usize {
    let len = line.len();
    let (i, tens) = first_max(&line[..len - 1]);
    let (_, ones) = first_max(&line[i + 1..]);
    let val = tens.to_digit(10).unwrap() * 10 + ones.to_digit(10).unwrap();
    val as usize
}

#[inline]
fn first_max(s: &str) -> (usize, char) {
    s.chars()
        .enumerate()
        .fold((0, '0'), |acc, (i, c)| if c > acc.1 { (i, c) } else { acc })
}
fn recursive_joltage(line: &str, mut remaining: usize, mut value: usize) -> (&str, usize, usize) {
    // Base case
    if remaining == 0 {
        return (line, remaining, value);
    }
    let len = line.len();
    // len = 6 (0..6), rem=4, len-rem=2  (0, 1, 2, 3, 4, 5), 4, 0
    // [0..=2] = (0,1,2) len 3, max=2, i=2, left (3, 4, 5) (new rem = 3)
    // len = 3 (1..4), rem=3, len-rem=0 (1,2,3), 3, 0
    // [0..=0] = (1,) len 1
    let (i, next_digit) = first_max(&line[..=len - remaining]);
    remaining -= 1;
    value = 10 * value
        + usize::try_from(
            next_digit
                .to_digit(10)
                .expect("The input strings should all be valid base-10 (radix-10) digits"),
        )
        .expect("A single digit should be a valid usize");
    recursive_joltage(&line[i + 1..], remaining, value)
}

pub fn part_1(input: &str) -> usize {
    input.lines().map(max_joltage).sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| recursive_joltage(line, 12, 0).2)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn joltage() {
        assert_eq!(max_joltage("1234"), 34);
    }
}
