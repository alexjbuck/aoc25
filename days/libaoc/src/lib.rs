use std::path::PathBuf;

use thiserror::Error;

const BASE_URI: &str = "https://adventofcode.com";
const CACHE_LOCATION: &str = "./.input";

#[derive(Debug, Error)]
pub enum AocError {
    #[error("failed to load input from cache: {0}")]
    Cache(#[from] std::io::Error),

    #[error("failed to download input from adventofcode.com: {0}")]
    Network(#[from] ureq::Error),
}

/// Get the input, either from a cached location or by downloading the input
/// # Errors
/// `AoCError::Cache` => The cache location exists but there was an error loading it
/// `AoCError::Network` => There was an issue downloading the input
/// # Panics
/// This function panics if the SESSION env var is not defined.
#[inline]
fn get_input(year: u16, day: u8, session: &str) -> Result<String, AocError> {
    match load_from_cache(year, day) {
        Ok(Some(input)) => Ok(input),
        _ => Ok(download_input(year, day, session)?),
    }
}

/// Attempt to load the input file from cache location.
/// # Errors
/// `std::io::Error` => The file does not exist or could not be read
#[inline]
fn load_from_cache(year: u16, day: u8) -> Result<Option<String>, std::io::Error> {
    let path = cached_path(year, day);
    std::fs::create_dir_all(path.parent().expect("Unable to define the cache directory"))
        .expect("Unable to create cache directory.");
    let input = std::fs::read_to_string(cached_path(year, day))?;
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

#[inline]
fn cached_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("{CACHE_LOCATION}/{year}/{day}.txt"))
}

/// This is a thin wrapper around the synchronous http request `ureq::get`
/// # Errors
/// This function can return a ureq:Error type.
#[inline]
fn download_input(year: u16, day: u8, session: &str) -> Result<String, ureq::Error> {
    let input = ureq::get(request_uri(year, day))
        .header("Cookie", format!("session={session}"))
        .call()?
        .body_mut()
        .read_to_string()?;
    cache_input(&input, year, day);
    Ok(input)
}

fn cache_input(input: &str, year: u16, day: u8) {
    let path = cached_path(year, day);
    std::fs::write(path, input).expect("Unable to cache input.");
}

#[inline]
fn request_uri(year: u16, day: u8) -> String {
    format!("{BASE_URI}/{year}/day/{day}/input")
}

#[derive(Debug, PartialEq, Eq)]
enum Part {
    One,
    Two,
    Both,
}

impl<S> From<S> for Part
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref();
        match value {
            "1" | "01" | "one" | "One" => Part::One,
            "2" | "02" | "two" | "Two" => Part::Two,
            _ => Part::Both,
        }
    }
}

/// Returns which part has been selected via command line args
///
/// >>> match value {
/// >>>     "1" | "01" | "one" | "One" => Part::One,
/// >>>     "2" | "02" | "two" | "Two" => Part::Two,
/// >>>     _ => Part::Both,
/// >>> }
fn part_from_args() -> Part {
    let arg = std::env::args().nth(1).unwrap_or_default();
    arg.into()
}

/// # Panics
/// This function panics if the input cannot be loaded
pub fn evaluate<F1: for<'a> Fn(&'a str) -> usize, F2: for<'a> Fn(&'a str) -> usize>(
    f1: F1,
    f2: F2,
    year: u16,
    day: u8,
) {
    dotenv::dotenv().ok();
    let session = std::env::var("SESSION").expect("SESSION env var not defined.");
    let part = part_from_args();
    let input = get_input(year, day, &session).expect("Unable to load input");
    if part == Part::One || part == Part::Both {
        println!("Part One: {}", f1(&input));
    }
    if part == Part::Two || part == Part::Both {
        println!("Part Two: {}", f2(&input));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_path_works() {
        let uri = request_uri(2025, 1);
        assert_eq!(uri, "https://adventofcode.com/2025/day/1/input");
    }
    #[test]
    fn cache_path_works() {
        let uri = cached_path(2025, 1);
        assert_eq!(uri, PathBuf::from("./.input/2025/1.txt"));
    }
    #[test]
    fn parts_from_str() {
        let strs = vec![
            "1", "2", "3", "one", "two", "three", "both", "One", "Two", "Three",
        ];
        let results = vec![
            Part::One,
            Part::Two,
            Part::Both,
            Part::One,
            Part::Two,
            Part::Both,
            Part::Both,
            Part::One,
            Part::Two,
            Part::Both,
        ];
        assert_eq!(strs.iter().map(Part::from).collect::<Vec<Part>>(), results);
    }
    #[test]
    fn parts_from_string() {
        let strs = vec![
            "1", "2", "3", "one", "two", "three", "both", "One", "Two", "Three",
        ];
        let strings: Vec<String> = strs.into_iter().map(String::from).collect();
        let results = vec![
            Part::One,
            Part::Two,
            Part::Both,
            Part::One,
            Part::Two,
            Part::Both,
            Part::Both,
            Part::One,
            Part::Two,
            Part::Both,
        ];
        assert_eq!(
            strings.iter().map(Part::from).collect::<Vec<Part>>(),
            results
        );
    }
}
