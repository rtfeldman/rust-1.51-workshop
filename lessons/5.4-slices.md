# Slices

Borrowing a Vec gets a *slice* which references its contents.

```rust
let i64_vec: Vec[i64] = vec![1, 2, 3];
let i64_slice: &[i64] = i64_vec.as_slice();
```

You

```rust
let i64_slice: &[i64] = &i64_vec;
```

> You can also do this by calling `.as_slice()` on the `Vec`, but it's most 
> common to borrow it instead because `&` is shorter than `.as_slice()`.
> They compile to the same thing.

## Array Slices

## String Slices
