#![doc = include_str!("../README.md")]

use std::{fmt, hash, marker::PhantomData};

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "sqlx")]
mod sqlx;

mod misc;

#[doc(hidden)]
pub extern crate paste;

/// Declares a new branded ID type.
///
/// Also introduces the `Branded<name>Tag` type tag (an unit struct) in the
/// scope.
///
/// Example:
///
/// ```
/// bty::brand!(
///     /// User ID type.
///     pub type UserId = i32;
/// );
///
/// /// User entity.
/// #[derive(Debug)]
/// pub struct User {
///     pub id: UserId,
///     pub username: String,
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! brand {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis type $tag:ident = $raw:ty ;
        )+
    ) => {
        $crate::paste::paste! {
            $(
                #[derive(Copy, Clone)]
                #[doc(hidden)]
                $vis struct [< Branded $tag Tag >];

                impl $crate::Tag for [< Branded $tag Tag >] {
                    const TAG_NAME: &'static str = stringify!($tag);
                }

                $(#[$attr])*
                $vis type $tag = $crate::Brand<[< Branded $tag Tag >], $raw>;
            )+
        }
    };
}

/// A generic type to construct branded types.
///
/// This type is generic over the `Tag` and `Raw` types. The `Raw` parameter
/// corresponds to the underlying type being branded. `Tag` is the type used to
/// discriminate different branded types, thus having no runtime representation.
///
/// Users shouldn't use the `Brand` type directly; using the [`brand`] macro is
/// more ergonomic since a type for the `Tag` discriminant is automatically
/// defined.
///
/// If the underlying `Raw` type implements some of Rust's common traits (such
/// as `Debug`, `PartialEq`, etc), so does `Brand`.
#[derive(Clone, Copy)]
pub struct Brand<Tag, Raw> {
    raw: Raw,
    tag: PhantomData<Tag>,
}

impl<Tag, Raw> Brand<Tag, Raw> {
    /// Returns the underlying branded value.
    #[must_use]
    pub fn into_raw(self) -> Raw {
        self.raw
    }

    /// Returns a reference to the underlying branded value.
    #[must_use]
    pub fn as_raw(&self) -> &Raw {
        &self.raw
    }

    /// Constructs a new branded value.
    ///
    /// This method's name is marked as "unchecked" since this operation may
    /// possibly lead to invalid branded values, according to the branded type.
    /// Hence, users should be careful when manually constructing branded
    /// values.
    #[must_use]
    pub fn unchecked_from_raw(raw: Raw) -> Self {
        Self {
            raw,
            tag: PhantomData,
        }
    }
}

// impl Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash

impl<Tag, Raw> fmt::Debug for Brand<Tag, Raw>
where
    Tag: crate::Tag,
    Raw: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(Tag::TAG_NAME).field(&self.raw).finish()
    }
}

impl<Tag, Raw: Default> Default for Brand<Tag, Raw> {
    fn default() -> Self {
        Self::unchecked_from_raw(Raw::default())
    }
}

impl<Tag, Raw: PartialEq> PartialEq for Brand<Tag, Raw>
where
    Raw: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<Tag, Raw: Eq> Eq for Brand<Tag, Raw> {}

impl<Tag, Raw: PartialOrd> PartialOrd for Brand<Tag, Raw> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<Tag, Raw: Ord> Ord for Brand<Tag, Raw> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<Tag, Raw: hash::Hash> hash::Hash for Brand<Tag, Raw> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

/// Internal trait implemented by brand type tags.
#[doc(hidden)]
pub trait Tag {
    /// The underlying tag name.
    const TAG_NAME: &'static str;
}

#[cfg(test)]
mod tests {
    super::brand!(
        type TestId = i32;
    );

    #[test]
    fn test_debug() {
        let id = TestId::unchecked_from_raw(10);
        let s = format!("{id:?}");
        assert_eq!(s, "TestId(10)");
    }
}
