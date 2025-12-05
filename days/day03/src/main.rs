/// # Panics
/// This function panics if the input cannot be read correctly
/// # Errors
/// This function returns an `AoCError` with variants "Cache" and "Network"
pub fn main() {
    let year = 2025;
    let day = 3;
    libaoc::evaluate(part_1, part_2, year, day);
}

#[inline]
fn first_max(s: &str) -> (usize, char) {
    s.chars()
        .enumerate()
        .fold((0, '0'), |acc, (i, c)| if c > acc.1 { (i, c) } else { acc })
}

fn max_joltage(line: &str) -> usize {
    let len = line.len();
    let (i, tens) = first_max(&line[..len - 1]);
    let (_, ones) = first_max(&line[i + 1..]);
    let val = tens.to_digit(10).unwrap() * 10 + ones.to_digit(10).unwrap();
    val as usize
}

fn part_1(input: &str) -> usize {
    input.lines().map(max_joltage).sum()
}

fn part_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn joltage() {
        assert_eq!(max_joltage("1234"), 34);
    }
}
