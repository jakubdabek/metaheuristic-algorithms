use crate::words::word::Word;
use itertools::Itertools;
use rand::distributions::{Bernoulli, Uniform};
use rand::prelude::*;
use rand_distr::Normal;
use std::borrow::Cow;
use std::cell::UnsafeCell;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::time::Instant;

pub type Value = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub word: Word,
    pub value: Value,
}

impl Solution {
    fn new(word: Word, value: Value) -> Self {
        Self { word, value }
    }
}

pub mod word;

#[derive(Debug)]
pub struct Dictionary<'a> {
    pub acceptable_words: HashSet<&'a [u8]>,
    pub available_letters: BTreeMap<u8, (u32, Value)>,
}

impl<'a> Dictionary<'a> {
    pub fn new<T: IntoIterator<Item = &'a [u8]>>(
        acceptable_words: T,
        available_letters: BTreeMap<u8, (u32, Value)>,
    ) -> Self {
        let acceptable_words = acceptable_words.into_iter().collect();
        // acceptable_words.sort();

        Self {
            acceptable_words,
            available_letters,
        }
    }

    pub fn slice_value(&self, word: &[u8]) -> Option<Value> {
        if !self.acceptable_words.contains(word) {
            return None;
        }

        let mut tmp = word.to_owned();
        tmp.sort_unstable();

        // eprintln!("{}", unsafe { std::str::from_utf8_unchecked(&tmp) });

        let mut value = 0;
        for (k, g) in &tmp.iter().group_by(|&&x| x) {
            let &(c, v) = self.available_letters.get(&k)?;
            let g_count = g.count() as u32;
            if g_count > c {
                return None;
            }

            value += g_count as u64 * v;
        }

        Some(value)
    }

    pub fn word_value(&self, word: &Word) -> Option<Value> {
        self.slice_value(word.as_slice())
    }
}

#[derive(Debug, Clone)]
struct WordWithValue {
    word: Word,
    value: Option<Value>,
}

impl WordWithValue {
    fn new(word: Word, dictionary: &Dictionary<'_>) -> Self {
        let value = dictionary.word_value(&word);

        Self { word, value }
    }
}

pub struct PrettyWord<'a> {
    word: &'a Word,
}

impl<'a> PrettyWord<'a> {
    fn new(word: &'a Word) -> Self {
        Self { word }
    }
}

impl fmt::Debug for PrettyWord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe {
            std::str::from_utf8_unchecked(self.word.as_slice())
        })
    }
}

pub struct PrettyWords<'a, T: IntoIterator<Item = &'a Word>> {
    words: UnsafeCell<Option<T>>,
}

impl<'a, T: IntoIterator<Item = &'a Word>> PrettyWords<'a, T> {
    fn new(words: T) -> Self {
        Self {
            words: UnsafeCell::new(Some(words)),
        }
    }
}

impl<'a, T: IntoIterator<Item = &'a Word>> fmt::Debug for PrettyWords<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                unsafe { &mut *self.words.get() }
                    .take()
                    .expect("PrettyWords can only be printed once")
                    .into_iter()
                    .map(PrettyWord::new),
            )
            .finish()
    }
}

const GENERATION_SIZE: usize = 5000;

pub fn search(initial: Vec<Word>, dictionary: Dictionary<'_>, time_limit: Instant) -> Solution {
    let generation_size = usize::max(GENERATION_SIZE, initial.len());
    let top_specimens = generation_size / 10;
    let mut population = Vec::with_capacity(generation_size * 2);
    population.extend(
        initial
            .into_iter()
            .map(|word| WordWithValue::new(word, &dictionary)),
    );

    eprintln!(
        "{:?}",
        PrettyWords::new(population.iter().map(|p| &p.word))
    );

    let rng = &mut thread_rng();

    let mut probabilities = Vec::with_capacity(population.capacity());
    let mut chosen_pairs = Vec::with_capacity(population.capacity());
    let mut tmp = Vec::with_capacity(population.capacity());
    let mut mutations = Vec::with_capacity(64);

    let letter_dist = Uniform::from(b'a'..=b'z');
    let lengthen_dist = Normal::new(1.0f64, 4.0).unwrap();
    let shorten_dist = Normal::new(1.0f64, 2.0).unwrap();
    // let recombination_choice_dist = Bernoulli::from_ratio(1, 3).unwrap();
    let mutation_choice_dist = Bernoulli::from_ratio(1, 10).unwrap();
    let mutation_probability_dist = Bernoulli::from_ratio(1, 20).unwrap();

    let mut iters = 0;
    while Instant::now() < time_limit {
        population.sort_unstable_by(|s1, s2| {
            Ord::cmp(
                &(std::cmp::Reverse(s1.value.unwrap_or(0)), s1.word.as_slice()),
                &(std::cmp::Reverse(s2.value.unwrap_or(0)), s2.word.as_slice()),
            )
        });
        population.dedup_by(|s1, s2| s1.word == s2.word);
        let last_acceptable = population.iter().find_position(|s| s.value.is_none());
        let last_good =
            last_acceptable.map_or(top_specimens, |(pos, _)| usize::min(pos, top_specimens));
        if let Some(p) = population.get_mut(last_good..) {
            p.shuffle(rng);
        }

        if iters % 100 == 10 {
            eprintln!(
                "{:?}",
                PrettyWords::new(population.iter().map(|p| &p.word).take(last_good))
            );
        }
        population.truncate(generation_size);

        probabilities.clear();
        probabilities.extend(population.iter().scan(0, |acc, s| {
            *acc += s.value.unwrap_or(1);
            Some(*acc)
        }));

        let sum = probabilities.last().unwrap();
        let dist = Uniform::new_inclusive(0, sum);
        chosen_pairs.clear();
        for _ in 0..generation_size / 2 {
            let s1 = probabilities
                .binary_search(&rng.sample(dist))
                .unwrap_or_else(|x| x);
            let s2 = probabilities
                .binary_search(&rng.sample(dist))
                .unwrap_or_else(|x| x);
            chosen_pairs.push((s1, s2));
        }

        for &(s1, s2) in &chosen_pairs {
            let s1 = &population[s1].word;
            let s2 = &population[s2].word;

            let i = rng.gen_range(0, s1.as_slice().len());
            let j = rng.gen_range(0, s2.as_slice().len());

            let word = Word::recombine(s1, s2, i, j);

            population.push(WordWithValue::new(word, &dictionary))
        }

        tmp.clear();

        let mut do_mutation = |s: &WordWithValue, rng: &mut ThreadRng| -> Option<WordWithValue> {
            let (s, mutated) = (Cow::Borrowed(&s.word), false);

            let (s, mutated) = if rng.sample(mutation_probability_dist) {
                let len = f64::ceil(rng.sample(shorten_dist).abs()) as usize;
                if len >= s.as_slice().len() {
                    (s, mutated)
                } else {
                    let pos = rng.gen_range(0, s.as_slice().len() - len);
                    (Cow::Owned(Word::mutate_shorten(&s, pos, len)), true)
                }
            } else {
                (s, mutated)
            };

            let (s, mutated) = if rng.sample(mutation_probability_dist) {
                (Cow::Owned(Word::mutate_shuffle(&s, rng)), true)
            } else {
                (s, mutated)
            };

            let (s, mutated) = if rng.sample(mutation_probability_dist) {
                let len = f64::ceil(rng.sample(lengthen_dist).abs()) as usize;
                let mut buf = Vec::with_capacity(len + s.as_slice().len());

                buf.extend(std::iter::repeat_with(|| rng.sample(letter_dist)).take(len));

                (Cow::Owned(Word::mutate_lengthen(&s, buf)), true)
            } else {
                (s, mutated)
            };

            let (s, mutated) = if !rng.sample(mutation_choice_dist) {
                mutations.clear();
                for (i, _) in s.as_slice().iter().enumerate() {
                    if rng.sample(mutation_probability_dist) {
                        mutations.push((i, rng.sample(letter_dist)));
                    }
                }
                if !mutations.is_empty() {
                    (
                        Cow::Owned(Word::mutate_replace_letters(&s, &mutations)),
                        true,
                    )
                } else {
                    (s, mutated)
                }
            } else {
                (s, mutated)
            };

            if mutated {
                Some(WordWithValue::new(Cow::into_owned(s), &dictionary))
            } else {
                None
            }
        };
        tmp.extend(population.iter().filter_map(|s| {
            let mut result = do_mutation(s, rng);
            loop {
                if !rng.sample(mutation_probability_dist) { break; }
                if let Some(s) = result {
                    result = do_mutation(&s, rng);
                } else {
                    break;
                }
            }
            result
        }));

        population.extend(tmp.drain(..));

        iters += 1;
    }

    println!("{}", iters);
    let best = population.iter().max_by_key(|&s| s.value).unwrap();
    let best_global = dictionary
        .acceptable_words
        .iter()
        .max_by_key(|w| dictionary.slice_value(w).unwrap_or(0))
        .unwrap();

    let best_value = dictionary.slice_value(best_global).unwrap_or(0);
    for i in (0..=best_value).rev().take(5) {
        let specimens = dictionary.acceptable_words
            .iter()
            .map(|w| (w, dictionary.slice_value(w).unwrap_or(0)))
            .filter(|&(_, v)| v == i);
        eprint!("global with value {}: ", i);
        for (specimen, _) in specimens {
            eprint!(
                "{} ",
                unsafe { std::str::from_utf8_unchecked(specimen) },
            );
        }
        eprintln!()
    }

    Solution::new(best.word.clone(), best.value.unwrap_or(0))
}
