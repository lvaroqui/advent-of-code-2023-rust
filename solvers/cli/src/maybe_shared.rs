use std::ops::Deref;

pub enum MaybeShared<T> {
    Shared(T),
    Separate(T, T),
}

impl<T> MaybeShared<T> {
    pub fn first(&self) -> &T {
        match self {
            MaybeShared::Shared(v) => v,
            MaybeShared::Separate(v, _) => v,
        }
    }

    pub fn second(&self) -> &T {
        match self {
            MaybeShared::Shared(v) => v,
            MaybeShared::Separate(_, v) => v,
        }
    }

    pub fn as_ref(&self) -> MaybeShared<&T>
    where
        T: Deref,
    {
        match self {
            MaybeShared::Shared(v) => MaybeShared::Shared(v),
            MaybeShared::Separate(first, second) => MaybeShared::Separate(first, second),
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> MaybeShared<U>
    where
        T: Deref,
    {
        match self {
            MaybeShared::Shared(v) => MaybeShared::Shared(f(v)),
            MaybeShared::Separate(first, second) => MaybeShared::Separate(f(first), f(second)),
        }
    }

    pub fn as_deref(&self) -> MaybeShared<&<T as Deref>::Target>
    where
        T: Deref,
    {
        self.as_ref().map(Deref::deref)
    }
}
