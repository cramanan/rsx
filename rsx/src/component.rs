// use std::fmt;

pub struct Children<V> {
    f: Box<dyn FnOnce() -> V>,
}

// impl<V> fmt::Debug for Children<V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Children").finish()
//     }
// }

impl<F, V> From<F> for Children<V>
where
    F: FnOnce() -> V + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

impl<V: Default + 'static> Default for Children<V> {
    fn default() -> Self {
        Self {
            f: Box::new(V::default),
        }
    }
}

impl<V> Children<V> {
    /// Instantiates the child view.
    pub fn call(self) -> V {
        (self.f)()
    }

    /// Create a new [`Children`] from a closure.
    pub fn new(f: impl FnOnce() -> V + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}
