use crate::Result;

pub trait BoolExt {
    fn if_else<T, F, G>(&self, t: F, f: G) -> T
    where
        F: FnOnce() -> T,
        G: FnOnce() -> T;
    fn to_option<T>(&self, t: Option<T>) -> Option<T>;
    fn to_option_with<T, F>(&self, t: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>;
    fn to_result<T, E>(&self, t: T, f: E) -> Result<T, E>;
    fn to_result_with<T, E, F, G>(&self, t: F, f: G) -> Result<T, E>
    where
        F: FnOnce() -> T,
        G: FnOnce() -> E;
}

impl BoolExt for bool {
    #[inline]
    fn if_else<T, F, G>(&self, t: F, f: G) -> T
    where
        F: FnOnce() -> T,
        G: FnOnce() -> T,
    {
        match self {
            true => t(),
            false => f(),
        }
    }

    #[inline]
    fn to_option<T>(&self, t: Option<T>) -> Option<T> {
        match self {
            true => t,
            false => None,
        }
    }

    #[inline]
    fn to_option_with<T, F>(&self, t: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            true => t(),
            false => None,
        }
    }

    #[inline]
    fn to_result<T, E>(&self, t: T, f: E) -> Result<T, E> {
        match self {
            true => Ok(t),
            false => Err(f),
        }
    }

    #[inline]
    fn to_result_with<T, E, F, G>(&self, t: F, f: G) -> Result<T, E>
    where
        F: FnOnce() -> T,
        G: FnOnce() -> E,
    {
        match self {
            true => Ok(t()),
            false => Err(f()),
        }
    }
}
