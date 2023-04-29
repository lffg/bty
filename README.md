# bty

[<img alt="github" src="https://img.shields.io/badge/github-lffg/bty-8da0cb?labelColor=555555&logo=github&style=for-the-badge" height="20">](https://github.com/lffg/bty)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bty.svg?color=fc8d62&logo=rust&style=for-the-badge" height="20">](https://crates.io/crates/bty)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bty-66c2a5?labelColor=555555&logo=docs.rs&style=for-the-badge" height="20">](https://docs.rs/bty)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/lffg/bty/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/lffg/bty/actions?query=branch:main)

**Streamlined definition and usage of branded types in Rust.**

This crate provides the `Brand` type and the `brand!` macro, which can be used
to declare and seamlessly use branded types in Rust.

```toml
[dependencies]
bty = "0.1"
```

_Supports rustc 1.60+_

## Example

The `brand!` macro may be used to declare branded types, which will be
discriminated based on the name of the type alias.

```rs
bty::brand!(
    type UserId = i32;
);
```

Instances of `UserId` may be constructed using one of the deserialization
implementations, such as the `serde` one or the `sqlx` one. Manually
instantiation, though unrecommended, can be done using the `unchecked_from_raw`
associated function.

## Rationale

It's not rare to have values that, although of the same type, belong to
different domains. For example, a web application could use the `i32` type to
represent both user ids and order ids.

While this may seem reasonable, since those different _domain types_ have the
same type, one could easily pass a user id to a function expecting an order id.

Since Rust's type system is nominal, this problem could be avoided by
introducing different types for each id. For example, one could have:

```rs
pub struct UserId(i32);

pub struct OrderId(i32);
```

Now the compiler statically ensures that a user id is never erroneously passed
in place of an order id. Nice!

Though this approach suits most cases, it gets unwieldy as the number of custom
id types grows since, for usability's sake, the type definition alone is rarely
sufficient. For example, to `Clone` or `Debug` a custom id, one must implement
those traits for _all_ of the custom types.

```rs
#[derive(Clone, Debug)]
pub struct UserId(i32);

#[derive(Clone, Debug)]
pub struct OrderId(i32);
```

The problem worsens as the number of uses for the id types grows. For example,
what about `serde` serialization and deserialization?

`bty` solves this problem by not having separate types for the branded types.
Instead, a single `Brand` type is used. Defined as `Brand<Tag, Raw>`, it is
generic over a `Tag` type, which discriminates values of different "brands"
(i.e., domains) and the underlying type, represented by `Raw`.

For most Rust's commonly used traits, if `Raw` implements it, then so does
`Brand`. This means if `Raw` implements `Clone` and `Debug`, `Brand<_, Raw>`
will also have them implemented.

Following the previous example, one could use `bty` and have:

```rs
bty::brand!(
    pub type UserId = i32;
    pub type OrderId = i32;
);
```

There's nothing special with the `i32` type. Just like manually defined structs,
any type may be used to construct a branded type.

## License

MIT License.
