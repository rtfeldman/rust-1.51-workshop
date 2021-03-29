# Type Parameters

## Option

Many languages have a special value named something like `null`,
`nil`, or `undefined` that can be used to indicate the absence of a
value. Rust does not have this concept. Instead, Rust has an `enum`
in the standard library called `Option`, which is defined like this:

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_i64: Option<i64> = Some(41);
let none_i64: Option<i64> = None;

let some_bool: Option<bool> = Some(true);
let none_bool: Option<bool> = None;
```

This is just like the enums we've seen before, except that the
value contained in the `Some` variant can change depending on the option.

## Type Parameter

The `<T>` in `Option<T>` defines a new *type parameter*. Once it's been declared, this `T` type parameter can be used as a normal type
anywhere else in the `enum` definition. 

When `Option` values are created, `T` can be replaced by other types.
For example, just like how we can have `Vec<i64>`, or `Vec<bool>`, 
the `<T>` type parameter in `Option<T>` means we can now have types 
like like `Option<i64>` or `Option<bool>`.
(`Vec` also has a type parameter in its definition.)

Type parameters can also be used in structs, in the same way.

## Option Example

`Vec` has a method called `pop` which removes the first element
from the vector and returns it, if possible. If the vector is empty,
it instead does nothing and returns `None`.

```rust
let mut nums: Vec<i64> = vec![2, 4, 6];
let last_elem: Option<i64> = nums.pop();

// last_elem == Some(6)
```

## Result

The `Result` type is similar to `Option`, except that it has two
type parameters. It's typically used for return values from a function
that can either succeed or fail with some error value.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let failure_reason: String = format!("Failed due to {}", reason);
let failure: Result<i64, String> = Err(failure_reason);
let success: Result<i64, String> = Ok(42);
```

> The `format!` macro works like `println!` except it returns a `String`
> instead of printing it to the console.

## Result Example

An example of a function that uses `Result` is `String::from_utf8`:

```rust
fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>
```

It takes a `Vec` of raw bytes and checks whether those bytes
represent a valid UTF-8 string. If they do, it returns a `Result::Ok`
variant with that validated `String` inside it. Otherwise, it
returns a `Result::Err` variant with a `FromUtf8Error` value inside
it. (`FromUtf8Error` is a struct containing details about what
was invalid.)
