use std::slice;
use std::ops;
use std::convert;


pub struct ObjectTable<V>(Vec<V>);

impl<V> ObjectTable<V> {
    pub fn new() -> ObjectTable<V> {
        ObjectTable(Vec::new())
    }

    pub fn iter(&self) -> ObjectTableIter<V> {
        ObjectTableIter {
            inner: self.0.iter(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&V> {
        if index < self.0.len() {
            Some(&self.0[index])
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[V] {
        self.0.as_slice()
    }
}

pub struct ObjectTableIter<'a, V> where V: 'a {
    inner: slice::Iter<'a, V>,
}

impl<'a, V> Iterator for ObjectTableIter<'a, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<V> ops::Index<usize> for ObjectTable<V> {
    type Output = V;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<V> convert::AsRef<[V]> for ObjectTable<V> {
    #[inline]
    fn as_ref(&self) -> &[V] {
        self.as_slice()
    }
}

