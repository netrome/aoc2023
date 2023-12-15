#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashMap<K, V, const N: usize>([TupleArray<K, V, N>; 256]);

impl<K, V, const N: usize> HashMap<K, V, N> {
    pub fn new() -> Self {
        Self(core::array::from_fn(|_| TupleArray::new()))
    }
}

impl<K: PartialEq + Hash, V, const N: usize> HashMap<K, V, N> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0[key.hash() as usize].get(key)
    }

    pub fn insert(&mut self, key: K, val: V) -> Result<Option<V>, Error> {
        self.0[key.hash() as usize].insert(key, val)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0[key.hash() as usize].remove(key)
    }

    pub fn arrays(&self) -> &[TupleArray<K, V, N>] {
        &self.0
    }
}

pub trait Hash {
    fn hash(&self) -> u8;
}

impl Hash for &str {
    fn hash(&self) -> u8 {
        hash_chars(self.chars())
    }
}

impl Hash for String {
    fn hash(&self) -> u8 {
        hash_chars(self.chars())
    }
}

fn hash_chars(chars: impl IntoIterator<Item = char>) -> u8 {
    chars
        .into_iter()
        .map(|c| c as u8)
        .fold(0, |acc, v| acc.wrapping_add(v).wrapping_mul(17))
}

use crate::tuplearray::Error;
use crate::tuplearray::TupleArray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing_works() {
        assert_eq!("HASH".hash(), 52);
    }

    #[test]
    fn insertion_works() {
        let mut map = HashMap::<_, _, 2>::new();
        map.insert("thirteen", 13).unwrap();
        map.insert("dolphin", 42).unwrap();
        map.insert("leet", 1337).unwrap();
        map.insert("teel", 7331).unwrap();
        map.insert("noon", 12).unwrap();
        map.insert("boobs", 80085).unwrap();

        assert_eq!(*map.get(&"leet").unwrap(), 1337);
        assert_eq!(*map.get(&"dolphin").unwrap(), 42);
        assert_eq!(map.insert("leet", 1338).unwrap().unwrap(), 1337);
    }

    #[test]
    fn removal_works() {
        let mut map = HashMap::<_, _, 2>::new();
        map.insert("leet", 1337).unwrap();
        map.insert("dolphin", 42).unwrap();

        assert!(map.remove(&"derp").is_none());
        assert_eq!(map.remove(&"dolphin").unwrap(), 42);

        assert_eq!(*map.get(&"leet").unwrap(), 1337);
        assert!(map.get(&"dolphin").is_none());
    }
}
