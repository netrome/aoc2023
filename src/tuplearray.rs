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
            None => todo!(),
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

    pub fn entry(&mut self, key: &K) -> Option<&mut V> {
        self.0
            .iter_mut()
            .filter_map(|item| item.as_mut())
            .find(|(k2, _)| k2 == key)
            .map(|(_, value)| value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Out of memory")]
    OutOfMemory,
}
