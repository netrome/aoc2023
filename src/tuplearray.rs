#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleArray<K, V, const N: usize>([Option<(K, V)>; N]);

impl<K: PartialEq, V, const N: usize> TupleArray<K, V, N> {
    pub fn new() -> Self {
        Self(core::array::from_fn(|_| None))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0
            .iter()
            .filter_map(|item| item.as_ref())
            .find(|(k2, _)| k2 == key)
            .map(|(_, value)| value)
    }

    pub fn insert(&mut self, key: K, mut val: V) -> Result<Option<V>, Error> {
        match self
            .0
            .iter_mut()
            .enumerate()
            .find(|(_, entry)| entry.as_ref().map(|(k, _)| k == &key).unwrap_or(true))
        {
            Some((_, Some(entry))) => {
                core::mem::swap(&mut val, &mut entry.1);
                Ok(Some(val))
            }
            Some((idx, None)) => {
                let slot = self.0.get_mut(idx).ok_or(Error::OutOfMemory)?;
                *slot = Some((key, val));
                Ok(None)
            }
            None => Err(Error::OutOfMemory),
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut res = None;
        self.0
            .iter_mut()
            .take_while(|entry| entry.is_some())
            .skip_while(|entry| entry.as_ref().map(|(k, _)| k != key).unwrap_or(true))
            .fold(&mut res, |acc, item| {
                core::mem::swap(acc, item);
                item
            });

        res.map(|(_, v)| v)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Out of memory")]
    OutOfMemory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_works() {
        let mut array = TupleArray::<_, _, 5>::new();
        array.insert("thirteen", 13).unwrap();
        array.insert("dolphin", 42).unwrap();
        array.insert("leet", 1337).unwrap();
        array.insert("teel", 7331).unwrap();
        array.insert("noon", 12).unwrap();
        array.insert("boobs", 80085).unwrap_err();

        assert_eq!(*array.get(&"leet").unwrap(), 1337);
        assert_eq!(*array.get(&"dolphin").unwrap(), 42);
        assert_eq!(array.insert("leet", 1338).unwrap().unwrap(), 1337);
    }

    #[test]
    fn removal_works() {
        let mut array = TupleArray::<_, _, 5>::new();
        array.insert("leet", 1337).unwrap();
        array.insert("dolphin", 42).unwrap();

        assert!(array.remove(&"derp").is_none());
        assert_eq!(array.remove(&"dolphin").unwrap(), 42);

        assert_eq!(*array.get(&"leet").unwrap(), 1337);
        assert!(array.get(&"dolphin").is_none());
    }
}
