use get_size::GetSize;
use get_size_derive::GetSize;
/// Allows to detach data and reattach it later.
use std::ops::{Deref, DerefMut};

#[derive(Debug, GetSize)]
pub(crate) struct Detach<T> {
    val: Option<Box<T>>,
}

impl<T> Default for Detach<T> {
    fn default() -> Self {
        Self { val: None }
    }
}

impl<T> Clone for Detach<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        if let Some(t) = &self.val {
            Detach {
                val: Some(t.clone()),
            }
        } else {
            Detach { val: None }
        }
    }
}

impl<T> Deref for Detach<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val.as_ref().expect("already detached")
    }
}

impl<T> DerefMut for Detach<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val.as_mut().expect("already detached")
    }
}

impl<T> Detach<T> {
    #[allow(dead_code)]
    pub(crate) fn new(val: T) -> Self {
        Self {
            val: Some(Box::new(val)),
        }
    }

    /// No data contained.
    #[allow(dead_code)]
    pub(crate) fn is_detached(&self) -> bool {
        self.val.is_none()
    }

    /// Detaches the data and links it with a key for reattaching.
    /// The key is not used here, but contains information for reattaching
    /// where ever this is used.
    ///
    /// Panics
    ///
    /// Panics if the data was already detached.
    pub(crate) fn detach<K: Copy>(&mut self, key: K) -> Detached<K, T> {
        let val = self.val.take().expect("already detached");
        Detached::new(key, val)
    }

    /// Reattaches the data.
    pub(crate) fn attach<K: Copy>(&mut self, detached: Detached<K, T>) {
        let Detached { key: _, val } = detached;
        self.val.replace(val);
    }

    /// Returns a reference to the data.
    ///
    /// Panics
    ///
    /// Panics if the data was detached.
    pub(crate) fn as_ref(&self) -> &T {
        self.val.as_ref().expect("already detached")
    }

    /// Returns a reference to the data.
    ///
    /// Panics
    ///
    /// Panics if the data was detached.
    pub(crate) fn as_mut(&mut self) -> &mut T {
        self.val.as_mut().expect("already detached")
    }

    /// Dissolves this container.
    ///
    /// Panics
    ///
    /// Panics if the data was detached.
    pub(crate) fn take(mut self) -> T {
        *self.val.take().expect("already detached")
    }
}

impl<T> From<T> for Detach<T> {
    fn from(val: T) -> Self {
        Self {
            val: Some(Box::new(val)),
        }
    }
}

/// Detached data. Implements Deref and DerefMut for transparent access
/// to the data. The attached key can be accessed with the key function.
#[derive(Debug)]
pub struct Detached<K, T> {
    key: K,
    val: Box<T>,
}

impl<K, T> Detached<K, T>
where
    K: Copy,
{
    fn new(key: K, val: Box<T>) -> Self {
        Self { key, val }
    }

    /// Extracts the key.
    pub fn key(det: &Detached<K, T>) -> K {
        det.key
    }
}

impl<K, T> Deref for Detached<K, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val.as_ref()
    }
}

impl<K, T> DerefMut for Detached<K, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::detach::Detach;

    #[test]
    fn test_detach() {
        let mut dd = Detach::new("fop");

        assert_eq!(dd.is_detached(), false);

        assert_eq!(*dd.as_ref(), "fop");
        assert_eq!(*dd.as_mut(), "fop");

        let d = dd.detach(0u32);

        assert_eq!(*d, "fop");
        assert_eq!(d.trim(), "fop");

        assert_eq!(dd.is_detached(), true);

        dd.attach(d);

        assert_eq!(dd.is_detached(), false);

        let tt = dd.take();

        assert_eq!(tt, "fop");
    }
}
