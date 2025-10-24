# binvec

[![Github](https://img.shields.io/badge/github-im--jababa%2Frust--binvec-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/im-jababa/rust-binvec)
[![Crates.io](https://img.shields.io/badge/crates.io-binvec-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/binvec)
[![Docs.rs](https://img.shields.io/badge/docs.rs-binvec-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/binvec)
[![Build](https://img.shields.io/github/actions/workflow/status/im-jababa/rust-binvec/rust.yml?branch=main&style=for-the-badge)](https://github.com/im-jababa/rust-binvec/actions?query=branch%3Amain)

`binvec` provides a fixed-size, stack-allocated bit vector implemented with const generics.
It packs boolean values densely into bytes so you can store large sets of flags without
the overhead of `bool` slices or heap allocations.

## Highlights
- Packs `L` boolean values into `(L + 7) / 8` bytes with zero heap usage.
- Const-generic type `Binvec<L, N>` gives compile-time guarantees about bit length.
- Ergonomic `binvec!` macro computes the minimal byte length for you.
- Constant-time indexing with `get`/`set` and checked or unchecked access variants.
- Methods for bulk operations such as `fill`, `count_ones`, `count_zeros`, and predicates for all-one/zero checks.
- Iterator support via `BinvecIter` for integration with the iterator ecosystem.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
binvec = "0.1"
```

> ℹ️ The crate currently uses only `core` APIs internally, but it is exported as a
> normal `std` crate. No additional features or dependencies are required.

## Quick Start

```rust
use binvec::*;

// Create a 12-bit binvec filled with zeros
let mut flags = binvec!(12, false);

// Flip a couple of bits
flags.set(3, true).unwrap();
flags.set(7, true).unwrap();

assert_eq!(flags.get(3), Some(true));
assert_eq!(flags.count_ones(), 2);
assert!(!flags.is_all_zero());
```

The `binvec!` macro is the recommended constructor. It expands to a const instantiation
of `Binvec<L, N>` where `N` is `(L + 7) >> 3`, ensuring the backing byte array is the
minimal size required to hold the requested bit length.

## API Overview

- `Binvec::<L, N>`: main container type. Use the macro unless you need to specify both
  const parameters manually.
- `get` / `set`: checked accessors that return `Option<bool>` or `Result<(), IndexOutOfBounds>`.
- `get_unchecked` / `set_unchecked`: unchecked versions for hot paths where you can
  guarantee the index is in range (calling these with an out-of-bounds index is UB).
- `fill`, `count_ones`, `count_zeros`, `is_all_one`, `is_all_zero`: helpers for bulk inspection.
- `iter`: yields `bool` values and integrates with `for` loops and iterator adapters.
- `BinvecIter`: explicit iterator type if you need to hold the iterator value.

The crate also exposes the `error::IndexOutOfBounds` error type so you can handle
checked writes ergonomically:

```rust
use binvec::*;

let mut bits = binvec!(8, false);
match bits.set(32, true) {
    Ok(()) => { /* success */ }
    Err(e) => eprintln!("write failed: {e}"),
}
```

## Safety & Performance Notes

- Checked access methods (`get`, `set`) are safe and return an error instead of panicking.
- Unchecked methods are `unsafe` because they can read or write past the backing array
  if misused; prefer the checked versions unless profiling shows you need the extra speed.
- All public const functions can be evaluated at compile time, making it easy to work
  with `Binvec` in `const` contexts or static initialisers.
