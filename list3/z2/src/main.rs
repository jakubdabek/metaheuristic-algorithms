use crate::words::word::Word;
use crate::words::{Dictionary, Solution, Value};
use itertools::Itertools;
use std::borrow::Cow;
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io::BufRead as _;
use std::ops::Add;
use std::time::{Duration, Instant};
use std::{fmt, io};

mod words;

fn do_search(
    initial: Vec<Word>,
    dictionary: Dictionary<'_>,
    duration_limit: Duration,
) -> (Word, Value, Duration) {
    let start = Instant::now();
    let Solution { word, value } =
        words::search(initial, dictionary, Instant::now().add(duration_limit));

    let elapsed = Instant::now().duration_since(start);

    (word, value, elapsed)
}

#[derive(Debug, Clone)]
struct MyError {
    msg: Cow<'static, str>,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for MyError {}

impl From<&'static str> for MyError {
    fn from(e: &'static str) -> Self {
        Self { msg: e.into() }
    }
}

impl From<String> for MyError {
    fn from(e: String) -> Self {
        Self { msg: e.into() }
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: e.to_string().into(),
        }
    }
}

fn main_interactive() -> Result<(), MyError> {
    let start = Instant::now();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let header = lines
        .next()
        .ok_or("not enough lines")??
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "invalid header")?;

    let (time, n, s) = match *header.as_slice() {
        [time, n, m] if time > 0 => (time, n, m),
        _ => return Err("invalid header".into()),
    };

    let mut letter_values = BTreeMap::new();
    // let mut letter_values = HashMap::new();

    for _ in 0..n {
        let line: String = lines.next().ok_or("not enough lines")??;
        let mut split = line.split_ascii_whitespace();
        let err_msg = "invalid character line";
        let c = split.next().ok_or(err_msg)?;
        let p = split.next().ok_or(err_msg)?;

        if split.next().is_some() {
            return Err(err_msg.into());
        }

        let c = c.bytes().next().ok_or(err_msg)?;
        let p = p.parse::<Value>().map_err(|_| err_msg)?;

        let (count, _) = letter_values
            .entry(c.to_ascii_lowercase())
            .or_insert((0, p));
        *count += 1;
    }

    let mut initial = Vec::with_capacity(s);
    initial.extend(lines.take(s).filter_map(Result::ok).map(Word::from_string));

    if initial.len() != s {
        return Err("not enough examples".into());
    }

    let acceptable_words = std::fs::read_to_string("dict.txt")?.to_lowercase();
    let acceptable_words = acceptable_words
        .as_bytes()
        .split(|&l| matches!(l, b'\n' | b'\r'))
        .filter(|s| !s.is_empty());
    let dictionary = Dictionary::new(acceptable_words, letter_values);

    eprintln!("initialization took {:?}", start.elapsed());
    eprintln!("dictionary: {:?}", dictionary.available_letters);
    eprintln!(
        "dictionary: {:?}",
        dictionary
            .acceptable_words
            .iter()
            .filter_map(|s| std::str::from_utf8(s).ok())
            .take(10)
            .collect_vec()
    );

    let (word, val, _elapsed) = do_search(initial, dictionary, Duration::from_secs(time as u64));

    println!("{}", val);
    eprintln!("{:?}", std::str::from_utf8(word.as_slice()));

    Ok(())
}

fn main() -> Result<(), MyError> {
    main_interactive()
}
