
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    l: usize,
    r: usize,
}

impl Range {
    fn is_disjoint(&self, other: Self) -> bool {
        other.r < self.l.saturating_sub(1) || self.r.saturating_add(1) < other.l
    }
    fn is_overlap(&self, other: Self) -> bool {
        !self.is_disjoint(other)
    }
    fn expand(&mut self, other: Self) {
        self.l = self.l.min(other.l);
        self.r = self.r.max(other.r);
    }
    fn size(&self) -> usize {
        self.r - self.l + 1
    }
}

#[derive(Clone, Debug, Default)]
struct RangeSet(Vec<Range>);

impl RangeSet {
    fn add(mut self, new_range: Range) -> Self {
        self.0.push(new_range);
        self.0.sort_unstable();
        let mut consolidated: Vec<Range> = Vec::with_capacity(self.0.len());

        for range in self.0 {
            match consolidated.last_mut() {
                Some(last) if last.is_overlap(range) => last.expand(range),
                _ => consolidated.push(range),
            }
        }
        Self(consolidated)
    }
    fn size(&self) -> usize {
        self.0.iter().map(Range::size).sum()
    }
}

impl From<(&str, &str)> for Range {
    fn from(value: (&str, &str)) -> Self {
        Self {
            l: value
                .0
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse value: {}", value.0)),
            r: value
                .1
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse value: {}", value.1)),
        }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let range = value
            .split_once('-')
            .unwrap_or_else(|| panic!("Unable to split range from {value}"));
        Self::from(range)
    }
}

pub fn part_1(input: &str) -> usize {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let mut top: Vec<_> = top.lines().map(Range::from).collect();
    top.sort_unstable_by(|a, b| a.l.cmp(&b.l));
    let bottom: Vec<usize> = bottom.lines().map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    for value in bottom {
        for range in &top {
            if range.l <= value && value <= range.r {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn part_2(input: &str) -> usize {
    let (top, _) = input.split_once("\n\n").unwrap();
    let ranges = top
        .lines()
        .map(Range::from)
        .fold(RangeSet::default(), RangeSet::add);
    ranges.size()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn example2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(part_2(input), 14);
    }
}
