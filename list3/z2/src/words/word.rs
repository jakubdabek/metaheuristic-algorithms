use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word {
    bytes: Box<[u8]>,
}

impl Word {
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
}

impl Word {
    pub fn new(bytes: Box<[u8]>) -> Self {
        Self { bytes }
    }

    pub fn from_string(word: String) -> Self {
        Self::new(word.into_bytes().into_boxed_slice())
    }
}

impl Word {
    pub fn recombine(s1: &Self, s2: &Self, i: usize, j: usize) -> Self {
        let len = i + s2.as_slice().len() - j;
        let mut bytes = Vec::with_capacity(len);
        bytes.extend_from_slice(&s1.as_slice()[..i]);
        bytes.extend_from_slice(&s2.as_slice()[j..]);

        Self {
            bytes: bytes.into_boxed_slice(),
        }
    }

    pub fn mutate_lengthen(specimen: &Self, mut letters: Vec<u8>) -> Self {
        let specimen = specimen.as_slice();
        let count = letters.len();
        letters.reserve_exact(specimen.len());
        letters.extend(std::iter::repeat(0).take(specimen.len()));
        letters.copy_within(..count, specimen.len());
        letters[..specimen.len()].copy_from_slice(specimen);

        Self {
            bytes: letters.into_boxed_slice(),
        }
    }

    pub fn mutate_shorten(specimen: &Self, pos: usize, len: usize) -> Self {
        let mut bytes = Vec::with_capacity(specimen.as_slice().len() - len);
        bytes.extend_from_slice(&specimen.as_slice()[..pos]);
        bytes.extend_from_slice(&specimen.as_slice()[pos + len..]);

        Self {
            bytes: bytes.into_boxed_slice(),
        }
    }

    pub fn mutate_shuffle(specimen: &Self, rng: &mut impl Rng) -> Self {
        let mut bytes = specimen.bytes.clone();

        bytes.shuffle(rng);

        Self { bytes }
    }

    pub fn mutate_replace_letters(specimen: &Self, positions: &[(usize, u8)]) -> Self {
        let mut bytes = specimen.bytes.clone();

        for &(pos, c) in positions {
            bytes[pos] = c;
        }

        Self { bytes }
    }
}
