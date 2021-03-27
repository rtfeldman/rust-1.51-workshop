# Integers

Rust has two different categories of numbers: floats and integers. Integers 
don't have a decimal component; they are either whole numbers 
(possibly negative) or zero.

Like floats, Rust integers also come in different sizes, and those sizes are
reflected in their types. Here's a function that takes an 8-bit integer and 
returns it multiplied by 3.

```rust
fn triple(integer: i8) -> i8 {
    return integer * 3;
}
```

An `i8` can store numbers as high as 127 and as low as -128. There are also
bigger sizes, like `i16`, `i32`, `i64`, and `i128`, which can store a wider
range of numbers. Like with `f32` and `f64`, smaller integer types use less 
memory, which can improve performance.

## Unsigned Integers

Rust also has *unsigned* integers, which are never negative.

```rust
fn triple(unsigned_integer: u8) -> u8 {
    let answer = unsigned_integer * 3;

    return answer;
}
```

We can pass this `triple` function numbers like 0, 1, or 2, but if we try to
call `triple(-1)`, it won't compile because unsigned integers can't be negative.
You can convert between signed and unsigned integers using `as`, and you can
convert between integers and floats using `as` too.

Unsigned integers are generally a better choice for values that semantically
should never be negative, like counts or dimensions. They can also store higher
numbers in the same amount of memory; `i8` goes from -128 to 127, but `u8` goes
from 0 to 255.

## Automatic returns

Instead of writing `return answer;` we can write `answer` without 
the semicolon at the end of the function. They do the same thing, because the
the last expression in a Rust function is automatically used as the function's
return value.

So these code snippets do the same thing:

```rust
fn triple(unsigned_integer: u8) -> u8 {
    let answer = unsigned_integer * 3;

    return answer;
}
```

```rust
fn triple(unsigned_integer: u8) -> u8 {
    let answer = unsigned_integer * 3;

    answer
}
```
