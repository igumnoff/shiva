//!
//! Defines the type AttrMap as container for different attribute-sets.
//! And there are a number of traits working with AttrMap to set
//! related families of attributes.
//!

use get_size::GetSize;
use std::mem::size_of;
use std::slice;
use string_cache::DefaultAtom;

/// Container type for attributes.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct AttrMap2 {
    keys: Vec<DefaultAtom>,
    values: Vec<Box<str>>,
}

impl GetSize for AttrMap2 {
    fn get_heap_size(&self) -> usize {
        self.keys.capacity() * size_of::<DefaultAtom>()
            + self.values.capacity() * size_of::<Box<str>>()
            + self.values.iter().map(|v| v.get_heap_size()).sum::<usize>()
    }
}

impl AttrMap2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AttrMap2 {
            keys: Default::default(),
            values: Default::default(),
        }
    }

    /// Are there any attributes?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.keys.shrink_to_fit();
        self.values.shrink_to_fit();
    }

    /// Add from Slice
    #[inline]
    pub fn add_all<'a, V: Into<String>, I: IntoIterator<Item = (&'a str, V)>>(&mut self, data: I) {
        for (k, v) in data {
            self.keys.push(DefaultAtom::from(k));
            self.values.push(v.into().into_boxed_str());
        }
    }

    /// Adds an attribute.
    #[inline]
    pub fn set_attr<S: Into<String>>(&mut self, name: &str, value: S) {
        let k = DefaultAtom::from(name);
        let v = value.into().into_boxed_str();
        if let Some(idx) = self.find_idx(&k) {
            self.keys[idx] = k;
            self.values[idx] = v;
        } else {
            self.keys.push(k);
            self.values.push(v);
        }
    }

    #[inline]
    pub(crate) fn push_attr<S: Into<String>>(&mut self, name: &str, value: S) {
        self.keys.push(DefaultAtom::from(name));
        self.values.push(value.into().into_boxed_str());
    }

    #[inline(always)]
    fn find_idx(&self, test: &DefaultAtom) -> Option<usize> {
        self.keys
            .iter()
            .enumerate()
            .find(|v| v.1 == test)
            .map(|v| v.0)
    }

    /// Removes an attribute.
    #[inline]
    pub fn clear_attr(&mut self, name: &str) -> Option<String> {
        let k = DefaultAtom::from(name);
        if let Some(idx) = self.find_idx(&k) {
            self.keys.remove(idx);
            Some(self.values.remove(idx).into_string())
        } else {
            None
        }
    }

    /// Returns the attribute.
    #[inline]
    pub fn attr(&self, name: &str) -> Option<&str> {
        let k = DefaultAtom::from(name);
        if let Some(idx) = self.find_idx(&k) {
            Some(&self.values[idx])
        } else {
            None
        }
    }

    /// Returns a property or a default.
    #[inline]
    pub fn attr_def<'a, 'b, S>(&'a self, name: &'b str, default: S) -> &'a str
    where
        S: Into<&'a str>,
    {
        let k = DefaultAtom::from(name);
        if let Some(idx) = self.find_idx(&k) {
            &self.values[idx]
        } else {
            default.into()
        }
    }

    pub fn iter(&self) -> AttrMapIter<'_> {
        From::from(self)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

/// Iterator for an AttrMap.
#[derive(Debug)]
pub struct AttrMapIter<'a> {
    it: slice::Iter<'a, DefaultAtom>,
    jt: slice::Iter<'a, Box<str>>,
}

impl<'a> From<&'a AttrMap2> for AttrMapIter<'a> {
    fn from(attrmap: &'a AttrMap2) -> Self {
        Self {
            it: attrmap.keys.iter(),
            jt: attrmap.values.iter(),
        }
    }
}

impl<'a> Iterator for AttrMapIter<'a> {
    type Item = (&'a DefaultAtom, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.it.next();
        let v = self.jt.next();

        match (k, v) {
            (Some(k), Some(v)) => Some((k, v)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attrmap2::AttrMap2;

    #[test]
    fn test_attrmap2() {
        let mut m = AttrMap2::new();

        m.add_all([("foo", "baz"), ("lol", "now"), ("ful", "uuu")]);
        assert_eq!(m.attr("foo").unwrap(), "baz");

        m.set_attr("lol", "loud!".to_string());
        assert_eq!(m.attr("lol").unwrap(), "loud!");

        m.clear_attr("ful");
        assert_eq!(m.attr("ful"), None);
    }
}
