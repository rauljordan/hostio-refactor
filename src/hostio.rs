macro_rules! wrap_hostio {
    ($(#[$meta:meta])* $name:ident $cache:ident $hostio:ident u32) => {
        wrap_hostio!(@simple $(#[$meta])* $name, $cache, $hostio, u32);
    };
    (@simple $(#[$meta:meta])* $name:ident, $cache:ident, $hostio:ident, $ty:ident) => {
        $(#[$meta])*
        pub fn $name() -> $ty {
            $cache.get()
        }
        pub(crate) static $cache: once_cell::sync::Lazy<hostio::CachedOption<$ty>> = once_cell::sync::Lazy::new(|| {
            hostio::CachedOption::new(|| 5u32 )
        });
    };
}

use atomic_cell::{atomic::AtomicPtr, AtomicCell};
pub(crate) use wrap_hostio;

/// Caches a value to avoid paying for hostio invocations.
pub(crate) struct CachedOption<T: Copy> {
    value: AtomicCell<Option<T>, AtomicPtr<T>>,
    loader: fn() -> T,
}

impl<T: Copy> CachedOption<T> {
    /// Creates a new [`CachedOption`], which will use the `loader` during `get`.
    pub fn new(loader: fn() -> T) -> Self {
        Self {
            value: AtomicCell::new(None),
            loader,
        }
    }

    /// Sets and overwrites the cached value.
    #[allow(dead_code)]
    pub fn set(&self, value: T) {
        self.value.store(Some(value));
    }

    /// Gets the value, writing it to the cache if necessary.
    pub fn get(&self) -> T {
        match self.value.load() {
            Some(v) => v,
            None => {
                let new_value = (self.loader)();
                self.value.store(Some(new_value));
                new_value
            }
        }
    }
}
