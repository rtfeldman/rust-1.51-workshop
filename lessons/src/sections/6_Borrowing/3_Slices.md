# Slices

A *slice* is a reference to some subset of a Vec's elements.

```rust
let i64_vec: Vec<i64> = vec![4, 6, 8, 10, 12];
let i64_slice1: &[i64] = &i64_vec[1..]; // 6,  8, 10, 12
let i64_slice2: &[i64] = &i64_vec[2..4]; // 8, 10
let i64_slice3: &[i64] = &i64_vec[3..]; // 10, 12
```

The slice does not make a copy of the data; rather, it references the same
data that's in the original `Vec`.

You can also get a slice of all the elements using `.as_slice()` like so:

```rust
let i64_vec: Vec<i64> = vec![1, 2, 3];
let i64_slice: &[i64] = i64_vec.as_slice();
```

## String Slices

You can similarly have slices of strings.

String slices always have the type `&str`, and you can get one by calling
`.as_str()` on a `String`.
