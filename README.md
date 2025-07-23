[![Latest Version](https://img.shields.io/crates/v/radix_fmt_ng.svg)](https://crates.io/crates/radix_fmt_ng)
[![Documentation](https://img.shields.io/badge/api-rustdoc-purple.svg)](https://docs.rs/radix_fmt_ng)

This crate adds a tool to format a number in an arbitrary base from 2 to 61.

This is a light crate, without any dependency.

For primitive signed integers (`i8` to `i128`, and `isize`), negative values are formatted as the two’s complement representation.

## Get started

Add the crate in the cargo manifest:

```toml
radix_fmt_ng = "1"
```

Import [`radix`](fn.radix.html) in scope, and you are ready to go:

```rust
use radix_fmt_ng::radix;
```

## Examples

```rust
use radix_fmt_ng::*;

let n = 35;

// Ouput: "z"
println!("{}", radix(n, 36));
```

## FAQ

- What if I want to use the capitalized letters as digits?

> No. If you did that the conversion from 36 to 61 would be meaningless.

- Why does the formatting of negative numbers give a weird result?

> Just as in the standard library, when a number is formatted in a non-decimal base, the two’s complement representation is used. That means that the number is casted to the unsigned version (for example, for an `i8` the following number is used: `n as u8`).
