//! This crate provides a single macro called `unwrap_let!`.
//!
//! `unwrap_let!` lets you easily write a pattern-matching let assignment that
//! panics when it fails. In essence, `unwrap_let!` is a let-else statement with
//! less boilerplate.
//!
//! `unwrap_let!` accepts a custom panic message similar to
//! [`std::assert!`](https://doc.rust-lang.org/std/macro.assert.html) macro.
//!
//! # Examples
//!
//! ```
//! let val = Some(123);
//!
//! unwrap_let!(Some(x) = val);
//! // `x` is defined
//! assert_eq!(x, 123);
//! ```
//!
//! ```
//! # use unwrap_let::unwrap_let;
//! enum Integer {
//!     Signed(i64),
//!     Unsigned(u64),
//! }
//!
//! fn negate(unchecked_num: Integer) -> Integer {
//!     unwrap_let!(Integer::Signed(num) = unchecked_num, "expected a signed integer");
//!     Integer::Signed(-num)
//! }
//! ```
#![no_std]

/// A Rust macro for quickly unwrapping a refutable pattern.
///
/// See the crate documentation for information on how to use this macro.
#[macro_export]
macro_rules! unwrap_let {
    ($pattern:pat = $expression:expr) => {
        let $pattern = $expression
        else {
            panic!("`{:?}` does not match `{}`", $expression, stringify!($pattern));
        };
    };
    ($pattern:pat = $expression:expr, $($msg:tt)+) => {
        let $pattern = $expression
        else {
            panic!($($msg)+);
        };
    };
}

mod test {
    #[test]
    fn test_unwrap_let() {
        let val = Some(123);

        unwrap_let!(Some(x) = val);
        assert_eq!(x, 123);
    }

    #[test]
    #[should_panic(expected = "`None` does not match `Some(_)`")]

    fn test_unwrap_let_panic() {
        let val: Option<()> = None;
        unwrap_let!(Some(_) = val);
    }

    #[test]
    #[should_panic(expected = "Custom panic message")]

    fn test_unwrap_let_panic_custom_message() {
        let val: Option<()> = None;
        unwrap_let!(Some(_) = val, "Custom panic message");
    }
}
