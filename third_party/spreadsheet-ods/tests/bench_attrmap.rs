#![allow(dead_code)]
#![allow(unreachable_pub)]

use crate::lib_test::Timing;
use criterion::black_box;

mod lib_test;

mod v1 {
    use get_size::GetSize;
    use string_cache::DefaultAtom;

    /// Container type for attributes.
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct AttrMap2 {
        // map: Option<HashMap<DefaultAtom, String>>,
        map: Vec<(DefaultAtom, String)>,
    }

    impl GetSize for AttrMap2 {
        fn get_heap_size(&self) -> usize {
            0
        }
    }

    impl AttrMap2 {
        #[allow(dead_code)]
        pub fn new() -> Self {
            AttrMap2 {
                map: Default::default(),
            }
        }

        /// Are there any attributes?
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }

        #[inline]
        pub fn shrink_to_fit(&mut self) {
            self.map.shrink_to_fit();
        }

        /// Add from Slice
        #[inline]
        pub fn add_all<'a, I: IntoIterator<Item = (&'a str, String)>>(&mut self, data: I) {
            self.map.extend(
                data.into_iter()
                    .map(|(name, value)| (DefaultAtom::from(name), value)),
            );
        }

        /// Adds an attribute.
        #[inline]
        pub fn set_attr<S: Into<String>>(&mut self, name: &str, value: S) {
            let v = (DefaultAtom::from(name), value.into());
            if let Some(idx) = self.find_idx(name) {
                self.map[idx] = v;
            } else {
                self.map.push(v);
            }
        }

        #[inline]
        pub(crate) fn push_attr<S: Into<String>>(&mut self, name: &str, value: S) {
            self.map.push((DefaultAtom::from(name), value.into()));
        }

        #[inline]
        fn find_idx(&self, name: &str) -> Option<usize> {
            let name = DefaultAtom::from(name);
            for (i, (k, _)) in self.map.iter().enumerate() {
                if name == *k {
                    return Some(i);
                }
            }
            None
        }

        /// Removes an attribute.
        #[inline]
        pub fn clear_attr(&mut self, name: &str) -> Option<String> {
            if let Some(idx) = self.find_idx(name) {
                Some(self.map.remove(idx).1)
            } else {
                None
            }
        }

        /// Returns the attribute.
        #[inline]
        pub fn attr(&self, name: &str) -> Option<&String> {
            if let Some(idx) = self.find_idx(name) {
                Some(&self.map[idx].1)
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
            if let Some(idx) = self.find_idx(name) {
                self.map[idx].1.as_str()
            } else {
                default.into()
            }
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.map.len()
        }
    }
}

mod v2 {
    use get_size::GetSize;
    use std::collections::HashMap;
    use string_cache::DefaultAtom;

    /// Container type for attributes.
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct AttrMap2 {
        map: Option<HashMap<DefaultAtom, String>>,
    }

    impl GetSize for AttrMap2 {
        fn get_heap_size(&self) -> usize {
            0
        }
    }

    impl AttrMap2 {
        #[allow(dead_code)]
        pub fn new() -> Self {
            AttrMap2 {
                map: Default::default(),
            }
        }

        /// Are there any attributes?
        pub fn is_empty(&self) -> bool {
            self.map.is_none()
        }

        /// Add from Slice
        pub fn add_all(&mut self, data: &[(&str, String)]) {
            let attr = self.map.get_or_insert_with(HashMap::new);
            for (name, value) in data {
                attr.insert(DefaultAtom::from(*name), value.to_string());
            }
        }

        /// Adds an attribute.
        pub fn set_attr<S: Into<String>>(&mut self, name: &str, value: S) {
            self.map
                .get_or_insert_with(HashMap::new)
                .insert(DefaultAtom::from(name), value.into());
        }

        /// Removes an attribute.
        pub fn clear_attr(&mut self, name: &str) -> Option<String> {
            if let Some(ref mut attr) = self.map {
                attr.remove(&DefaultAtom::from(name))
            } else {
                None
            }
        }

        /// Returns the attribute.
        pub fn attr(&self, name: &str) -> Option<&String> {
            if let Some(ref prp) = self.map {
                prp.get(&DefaultAtom::from(name))
            } else {
                None
            }
        }

        /// Returns a property or a default.
        pub fn attr_def<'a, 'b, S>(&'a self, name: &'b str, default: S) -> &'a str
        where
            S: Into<&'a str>,
        {
            if let Some(ref prp) = self.map {
                if let Some(value) = prp.get(&DefaultAtom::from(name)) {
                    value.as_ref()
                } else {
                    default.into()
                }
            } else {
                default.into()
            }
        }
    }
}

mod v3 {
    use get_size::GetSize;
    use string_cache::DefaultAtom;

    /// Container type for attributes.
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct AttrMap2 {
        keys: Vec<DefaultAtom>,
        values: Vec<String>,
    }

    impl GetSize for AttrMap2 {
        fn get_heap_size(&self) -> usize {
            0
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
        pub fn add_all<'a, I: IntoIterator<Item = (&'a str, String)>>(&mut self, data: I) {
            for (k, v) in data {
                self.keys.push(DefaultAtom::from(k));
                self.values.push(v);
            }
        }

        /// Adds an attribute.
        #[inline]
        pub fn set_attr<S: Into<String>>(&mut self, name: &str, value: S) {
            let k = DefaultAtom::from(name);
            let v = value.into();
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
            self.values.push(value.into());
        }

        #[inline]
        fn find_idx(&self, test: &DefaultAtom) -> Option<usize> {
            for (i, key) in self.keys.iter().enumerate() {
                if test == key {
                    return Some(i);
                }
            }
            None
        }

        /// Removes an attribute.
        #[inline]
        pub fn clear_attr(&mut self, name: &str) -> Option<String> {
            let k = DefaultAtom::from(name);
            if let Some(idx) = self.find_idx(&k) {
                self.keys.remove(idx);
                Some(self.values.remove(idx))
            } else {
                None
            }
        }

        /// Returns the attribute.
        #[inline]
        pub fn attr(&self, name: &str) -> Option<&String> {
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
                self.values[idx].as_str()
            } else {
                default.into()
            }
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.keys.len()
        }
    }
}

mod v4 {
    use get_size::GetSize;
    use string_cache::DefaultAtom;

    /// Container type for attributes.
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct AttrMap2 {
        keys: Vec<DefaultAtom>,
        values: Vec<Box<str>>,
    }

    impl GetSize for AttrMap2 {
        fn get_heap_size(&self) -> usize {
            0
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
        pub fn add_all<'a, I: IntoIterator<Item = (&'a str, String)>>(&mut self, data: I) {
            for (k, v) in data {
                self.keys.push(DefaultAtom::from(k));
                self.values.push(v.into_boxed_str());
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

        #[inline]
        fn find_idx(&self, test: &DefaultAtom) -> Option<usize> {
            for (i, key) in self.keys.iter().enumerate() {
                if test == key {
                    return Some(i);
                }
            }
            None
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
                self.values[idx].as_ref()
            } else {
                default.into()
            }
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.keys.len()
        }
    }
}

// #[test]
fn test_all() {
    test_attrmap1();
    test_attrmap2();
    test_attrmap3();
    test_attrmap4();
}

fn test_attrmap1() {
    let mut t0 = Timing::<()>::default().name("vec").skip(10).runs(100000);

    let _ = t0.run_nf(|| {
        let mut a = v1::AttrMap2::new();

        for v in 0..45 {
            a.set_attr(&v.to_string(), "value");
        }
        for v in 23..25 {
            a.clear_attr(&v.to_string());
        }
        for v in 17..39 {
            let _ = black_box(a.attr(&v.to_string()));
        }
    });

    println!("{}", t0);
}

fn test_attrmap2() {
    let mut t0 = Timing::<()>::default()
        .name("hashmap")
        .skip(10)
        .runs(100000);

    let _ = t0.run_nf(|| {
        let mut a = v2::AttrMap2::new();

        for v in 0..45 {
            a.set_attr(&v.to_string(), "value");
        }
        for v in 23..25 {
            a.clear_attr(&v.to_string());
        }
        for v in 17..39 {
            let _ = black_box(a.attr(&v.to_string()));
        }
    });

    println!("{}", t0);
}

fn test_attrmap3() {
    let mut t0 = Timing::<()>::default().name("vec2").skip(10).runs(100000);

    let _ = t0.run_nf(|| {
        let mut a = v3::AttrMap2::new();

        for v in 0..45 {
            a.set_attr(&v.to_string(), "value");
        }
        for v in 23..25 {
            a.clear_attr(&v.to_string());
        }
        for v in 17..39 {
            let _ = black_box(a.attr(&v.to_string()));
        }
    });

    println!("{}", t0);
}

fn test_attrmap4() {
    let mut t0 = Timing::<()>::default().name("vec3").skip(10).runs(100000);

    let _ = t0.run_nf(|| {
        let mut a = v4::AttrMap2::new();

        for v in 0..45 {
            a.set_attr(&v.to_string(), "value");
        }
        for v in 23..25 {
            a.clear_attr(&v.to_string());
        }
        for v in 17..39 {
            let _ = black_box(a.attr(&v.to_string()));
        }
    });

    println!("{}", t0);
}
