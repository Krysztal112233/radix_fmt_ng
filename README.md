This crate adds a tool to format a number in a base
in range `[2, 36]`, since this feature was removed from the
standard library.

Add the crate, import [`radix`](fn.radix.html) in scope,
and you are ready to go:

```rust
extern crate radix_fmt;
use radix_fmt::radix;
```

Note that you also have one specific function for each radix that does not
already exists in the standard library, *e.g.* [`radix_3`](fn.radix_3.html)
to format a number in base 3.

# Examples:

```rust
use radix_fmt::radix;
use std::fmt::Write;

let n = 35;

// Ouput: "z"
println!("{}", radix(n, 36));
// Same ouput: "z"
println!("{}", radix_36(n));
```

You can use the *alternate* modifier to capitalize the letter-digits:

```rust
use radix_fmt::radix;
use std::fmt::Write;

let n = 35;

// Ouput: "Z"
println!("{:#}", radix(n, 36));
```

# FAQ:

* Which digits are used when the base is superior to `10`?

    > This crate uses the letters in alphabetic order. That is
    why the maximum base is 36: it uses all the digits and all
    the letters of the alphabet.

* Among the functions that format in a specific base, why are some missing?
For example there are `radix_7` and `radix_9`, but not `radix_8`.

    > All the numbers in range `[2, 36]` are represented except
    `2`, `8`, `10` and `16` because they already exist in the
    standard library through binary, octal, decimal (regular) and
    hexadecimal formatting.

* What if I want to use the capitalized letters as digits?

    > Use the *alternate* modifier `{:#}`.
