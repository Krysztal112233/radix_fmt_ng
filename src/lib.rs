//! This crate adds a tool to format a number in an arbitrary base from 2 to 36.
//!
//! Add the crate, import [`radix`](fn.radix.html) in scope,
//! and you are ready to go:
//!
//! ```
//! extern crate radix_fmt;
//! use radix_fmt::radix;
//! ```
//!
//! Note that you also have one specific function for each radix that does not
//! already exists in the standard library, *e.g.* [`radix_3`](fn.radix_3.html)
//! to format a number in base 3.
//!
//! # Examples:
//!
//! ```rust
//! use radix_fmt::*;
//! use std::fmt::Write;
//!
//! let n = 35;
//!
//! // Ouput: "z"
//! println!("{}", radix(n, 36));
//! // Same ouput: "z"
//! println!("{}", radix_36(n));
//! ```
//!
//! You can use the *alternate* modifier to capitalize the letter-digits:
//!
//! ```rust
//! use radix_fmt::radix;
//! use std::fmt::Write;
//!
//! let n = 35;
//!
//! // Ouput: "Z"
//! println!("{:#}", radix(n, 36));
//! ```

#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
/// A struct to format a number in an arbitrary radix.
///
/// # Example:
///
/// ```rust
/// use radix_fmt::Radix;
/// use std::fmt::Write;
///
/// let n = 15;
/// let mut s = String::new();
///
/// write!(&mut s, "{}", Radix::new(n, 3));
/// assert_eq!(s, "120"); // 15 is "120" in base 3
/// ```
pub struct Radix<T> {
    n: T,
    base: u32,
}

macro_rules! impl_display {
    ($i: ty, $u: ty) => {
        impl Display for Radix<$i> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                fn do_format(mut n: $u, base: u32, f: &mut Formatter) -> Result {
                    // BUF_SIZE is 128 because u128::max_value() in base 2 takes
                    // 128 digits to write.
                    const BUF_SIZE: usize = 128;
                    let mut buffer = [0 as char; BUF_SIZE];
                    let mut index = BUF_SIZE - 1;
                    let b = base as $u;

                    for c in buffer.iter_mut().rev() {
                        let digit = (n % b) as u8;
                        *c = match digit {
                            0...9 => (digit + '0' as u8) as char,
                            10...35 =>
                                if f.alternate() {
                                    (digit + 'A' as u8 - 10) as char
                                } else {
                                    (digit + 'a' as u8 - 10) as char
                                },
                            _ => unreachable!("Digit is not in range [0..36]"),
                        };
                        n /= b;
                        if n == 0 {
                            break;
                        }
                        index -= 1;
                    }

                    for c in buffer[index..].iter() {
                        write!(f, "{}", c)?;
                    }

                    Ok(())
                }
                match self.base {
                    2 => write!(f, "{:b}", self.n),
                    8 => write!(f, "{:o}", self.n),
                    10 => write!(f, "{}", self.n),
                    16 => write!(f, "{:X}", self.n),
                    base => do_format(self.n as $u, base, f),
                }
            }
        }
    };
}

impl_display!(i8, u8);
impl_display!(u8, u8);

impl_display!(i16, u16);
impl_display!(u16, u16);

impl_display!(i32, u32);
impl_display!(u32, u32);

impl_display!(i64, u64);
impl_display!(u64, u64);

impl_display!(isize, usize);
impl_display!(usize, usize);

impl<T> Radix<T> {
    /// Create a new displayable number from number and base.
    /// The base must be in the range [2, 36].
    pub fn new(n: T, base: u32) -> Self {
        assert!(base >= 2 && base <= 36);
        Radix { n, base }
    }
}

/// A helper for creating a new formatter from
/// [`Radix::new`](struct.Radix.html#method.new).
///
/// # Example:
///
/// ```rust
/// use radix_fmt::radix;
///
/// // Similar to println!("{}", Radix::new(123, 10));
/// // Prints: "123"
/// println!("{}", radix(123, 10));
/// ```
pub fn radix<T>(n: T, base: u32) -> Radix<T> { Radix::new(n, base) }
/// Formats a number in base 3.
pub fn radix_3<T>(n: T) -> Radix<T> { Radix::new(n, 3) }
/// Formats a number in base 4.
pub fn radix_4<T>(n: T) -> Radix<T> { Radix::new(n, 4) }
/// Formats a number in base 5.
pub fn radix_5<T>(n: T) -> Radix<T> { Radix::new(n, 5) }
/// Formats a number in base 6.
pub fn radix_6<T>(n: T) -> Radix<T> { Radix::new(n, 6) }
/// Formats a number in base 7.
pub fn radix_7<T>(n: T) -> Radix<T> { Radix::new(n, 7) }
/// Formats a number in base 9.
pub fn radix_9<T>(n: T) -> Radix<T> { Radix::new(n, 9) }
/// Formats a number in base 11.
pub fn radix_11<T>(n: T) -> Radix<T> { Radix::new(n, 11) }
/// Formats a number in base 12.
pub fn radix_12<T>(n: T) -> Radix<T> { Radix::new(n, 12) }
/// Formats a number in base 13.
pub fn radix_13<T>(n: T) -> Radix<T> { Radix::new(n, 13) }
/// Formats a number in base 14.
pub fn radix_14<T>(n: T) -> Radix<T> { Radix::new(n, 14) }
/// Formats a number in base 15.
pub fn radix_15<T>(n: T) -> Radix<T> { Radix::new(n, 15) }
/// Formats a number in base 17.
pub fn radix_17<T>(n: T) -> Radix<T> { Radix::new(n, 17) }
/// Formats a number in base 18.
pub fn radix_18<T>(n: T) -> Radix<T> { Radix::new(n, 18) }
/// Formats a number in base 19.
pub fn radix_19<T>(n: T) -> Radix<T> { Radix::new(n, 19) }
/// Formats a number in base 20.
pub fn radix_20<T>(n: T) -> Radix<T> { Radix::new(n, 20) }
/// Formats a number in base 21.
pub fn radix_21<T>(n: T) -> Radix<T> { Radix::new(n, 21) }
/// Formats a number in base 22.
pub fn radix_22<T>(n: T) -> Radix<T> { Radix::new(n, 22) }
/// Formats a number in base 23.
pub fn radix_23<T>(n: T) -> Radix<T> { Radix::new(n, 23) }
/// Formats a number in base 24.
pub fn radix_24<T>(n: T) -> Radix<T> { Radix::new(n, 24) }
/// Formats a number in base 25.
pub fn radix_25<T>(n: T) -> Radix<T> { Radix::new(n, 25) }
/// Formats a number in base 26.
pub fn radix_26<T>(n: T) -> Radix<T> { Radix::new(n, 26) }
/// Formats a number in base 27.
pub fn radix_27<T>(n: T) -> Radix<T> { Radix::new(n, 27) }
/// Formats a number in base 28.
pub fn radix_28<T>(n: T) -> Radix<T> { Radix::new(n, 28) }
/// Formats a number in base 29.
pub fn radix_29<T>(n: T) -> Radix<T> { Radix::new(n, 29) }
/// Formats a number in base 30.
pub fn radix_30<T>(n: T) -> Radix<T> { Radix::new(n, 30) }
/// Formats a number in base 31.
pub fn radix_31<T>(n: T) -> Radix<T> { Radix::new(n, 31) }
/// Formats a number in base 32.
pub fn radix_32<T>(n: T) -> Radix<T> { Radix::new(n, 32) }
/// Formats a number in base 33.
pub fn radix_33<T>(n: T) -> Radix<T> { Radix::new(n, 33) }
/// Formats a number in base 34.
pub fn radix_34<T>(n: T) -> Radix<T> { Radix::new(n, 34) }
/// Formats a number in base 35.
pub fn radix_35<T>(n: T) -> Radix<T> { Radix::new(n, 35) }
/// Formats a number in base 36.
pub fn radix_36<T>(n: T) -> Radix<T> { Radix::new(n, 36) }
