use const_fnv1a_hash::fnv1a_hash_str_64;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Essentially a `HashMap<&str, isize>` but hashed to reduce size and
/// increase performance in contexts where the key is known at compile time.
#[derive(Debug, Serialize, Deserialize)]
pub struct Offsets(HashMap<u64, HashMap<u64, isize>>);

impl Offsets {
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[inline]
    pub fn add_offset(&mut self, symbol: &str, hash: &str, offset: isize) {
        self.0.entry(fnv1a_hash_str_64(symbol)).or_default().insert(fnv1a_hash_str_64(hash), offset);
    }

    #[inline]
    pub fn get_offset(&self, symbol: &str, hash: Option<&str>) -> Option<isize> {
        let offsets = self.0.get(&fnv1a_hash_str_64(symbol))?;

        match hash {
            Some(hash) => offsets.get(&fnv1a_hash_str_64(hash)).copied(),
            None => offsets.values().next().copied(),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
