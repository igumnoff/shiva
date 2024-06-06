use crate::HashMap;
use get_size::GetSize;
use std::borrow::Cow;

pub(crate) mod format;
pub(crate) mod parse;
pub(crate) mod read;
pub(crate) mod write;

mod xmlwriter;

#[derive(Clone, Debug)]
pub(crate) struct NamespaceMap {
    map: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl GetSize for NamespaceMap {}

impl NamespaceMap {
    pub(crate) fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub(crate) fn insert(&mut self, k: String, v: String) {
        self.map.insert(Cow::Owned(k), Cow::Owned(v));
    }

    pub(crate) fn insert_str(&mut self, k: &'static str, v: &'static str) {
        self.map.insert(Cow::Borrowed(k), Cow::Borrowed(v));
    }

    pub(crate) fn entries(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Cow<'static, str>)> {
        self.map.iter()
    }
}
