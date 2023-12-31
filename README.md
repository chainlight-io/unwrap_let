# `unwrap_let!`

A Rust macro for quickly unwrapping a refutable pattern.

## Install

Add `unwrap_let` to your dependency by running `cargo add unwrap_let` or editing `Cargo.toml`:

```toml
[dependencies]
unwrap_let = "0.1.0"
```

Then, import `unwrap_let` macro either by `[macro_use]` or a `use` statement.

```rust
// Option 1:
// Having this line in `lib.rs` or `main.rs` allows you to use `unwrap_let` in
// any file in your project without additional `use` statement.
#[macro_use]
extern crate unwrap_let;
// Option 2:
// Or, you can import it in each file that uses this macro.
use unwrap_let::unwrap_let;
```

## Example

```rust
let val = Some(123);

unwrap_let!(Some(x) = val);
// `x` is defined
assert_eq!(x, 123);
```

```rust
enum Integer {
    Signed(i64),
    Unsigned(u64),
}

fn negate(unchecked_num: Integer) -> Integer {
    unwrap_let!(Integer::Signed(num) = unchecked_num, "expected a signed integer");
    Integer::Signed(-num)
}
```

Here is another example in our production test code.

```rust
fn test_multi_dimensional_array() {
    // ... parsing & lifting ...

    unwrap_let!(Some(TypeName::Array(dim1)) = variable_dimensional.type_name());
    unwrap_let!(TypeName::Array(dim2) = dim1.base());
    unwrap_let!(TypeName::Array(dim3) = dim2.base());
    unwrap_let!(TypeName::Array(dim4) = dim3.base());

    assert!(dim1.array_len() == Some(U256::from(3)));
    assert!(dim2.array_len() == Some(U256::from(2)));
    assert!(dim3.array_len() == None);
    assert!(dim4.array_len() == Some(U256::from(1)));

    // ... more checks ...
}
```

## Minimum Supported Rust Version

`unwrap_let` internally uses "let-else" statement, which was introduced in Rust 1.65 (Nov 2022).

## License

This project is licensed under the [MIT license](./LICENSE).
