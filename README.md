# Magic Space

![Logo](./images/stars.jpg) ~Yes, It's AI generated.~

[![Crates.io](https://img.shields.io/crates/v/magic-space.svg)](https://crates.io/crates/magic-space)
[![Docs.rs](https://docs.rs/magic-space/badge.svg)](https://docs.rs/magic-space)
[![License](https://img.shields.io/crates/l/magic-space.svg)](

Derive macro for Solana program state structs that automatically calculates the space required for the struct.

Code mostly taken from `@coral-xyz/anchor` project, modified to work without the anchor crate.

Source:

- https://github.com/coral-xyz/anchor/blob/master/lang/derive/space/src/lib.rs

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
magic_space = "^0.1.0"
```

Or cargo add:

```sh
$ cargo add magic-space
```

## Usage

```rust
use magic_space::*;

#[derive(MagicSpace)]
struct SomeAccount {
    apples: u32,
    oranges: u32,
    #[max_len(10)]
    others: Vec<u32>,
    #[max_len(10)]
    name: String,
    #[max_len(10, 5, 5)]
    some_other: Vec<Vec<Vec<u8>>>,
    #[max_len(10)]
    e: Vec<Option<SomeEnum>>,
}
```

If you have dynamic allocation, you can still use the max_len attribute as follows: `#[max_len(0)]` and avoid worrying about the vector length, that way you can apply the following pattern:

```rust
#[derive(MagicSpace)]
pub struct DynamicSizeVecPattern {
    #[max_len(0)]
    pub data: Vec<Item>,
}

impl DynamicSizeVecPattern {
    pub fn size(vec_len: usize, str_len: usize) -> usize {
        Self::MAGIC_SPACE + (vec_len * Item::size(str_len))
    }
}

#[derive(MagicSpace)]
pub struct Item {
    pub data: u64,
    #[max_len(0)]
    pub name: String,
}
```

## License

Apache 2.0 [LICENSE](LICENSE)
