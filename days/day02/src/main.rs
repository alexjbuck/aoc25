/// Use `libaoc` to run `part_1` and `part_2` according to command line arguments
/// # Panics
/// This function panics if the input cannot be read correctly
/// # Errors
/// This function returns an `AoCError` with variants "Cache" and "Network"
pub fn main() {
    let year = 2025;
    let day = 2;
    libaoc::evaluate(part_1, part_2, year, day);
}
/// Parse the range strings "123-456" into 2 usize values.
fn parse_range(range: &str) -> (usize, usize) {
    let range: Vec<_> = range
        .split('-')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    (range[0], range[1])
}

/// Test if a value has a repeating digit pattern of length 2
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_double(x: &usize) -> bool {
    let x = format!("{x}");
    let len = x.len();
    if len.is_multiple_of(2) {
        let (left, right) = x.split_at(len / 2);
        left == right
    } else {
        false
    }
}

/// Test if a value has a repeating digit pattern of length n
fn repeats_n(s: &str, n: usize) -> bool {
    let f = s.len() / n;
    let base = s.get(..f).expect("f should be smaller than length");
    let cmp = base.repeat(n);
    s == cmp
}

/// Assess is a value has a repeating digit pattern of length n>=2
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_repeat(x: &usize) -> bool {
    let s = format!("{x}");
    let len = s.len();
    (2..=len)
        .take_while(|n| n * n <= *x)
        .filter(|n| len.is_multiple_of(*n))
        .any(|f| repeats_n(&s, f))
}

/// Find values that repeat n=2 times in pattern
fn find_doubles(range: (usize, usize)) -> Vec<usize> {
    let (a, b) = range;
    (a..=b).filter(is_double).collect()
}

/// Find values that repeat n>=2 times in pattern
fn find_repeats(range: (usize, usize)) -> Vec<usize> {
    let (a, b) = range;
    (a..=b).filter(is_repeat).collect()
}

fn part_1(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .map(parse_range)
        .flat_map(find_doubles)
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .map(parse_range)
        .flat_map(find_repeats)
        .sum()
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test {
    use super::*;
    #[test]
    fn test_repeat() {
        assert!(is_repeat(&11));
        assert!(is_repeat(&111));
        assert!(is_repeat(&1111));
        assert!(is_repeat(&12_12_12_12_12));
        assert!(is_repeat(&123_123_123_123));
        assert!(is_repeat(&1234_1234_1234_1234));
        assert!(is_repeat(&1234_5678_1234_5678));
    }
    #[test]
    fn test_not_repeat() {
        assert!(!is_repeat(&10));
        assert!(!is_repeat(&101));
        assert!(!is_repeat(&1011));
        assert!(!is_repeat(&11_12_12_12_12));
        assert!(!is_repeat(&113_123_123_123));
        assert!(!is_repeat(&1134_1234_1234_1234));
        assert!(!is_repeat(&1134_5678_1234_5678));
    }
}
