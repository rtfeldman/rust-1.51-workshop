# Arrays

In Rust, an array is a collection where each element has the same type, and
the number of elements never changes at runtime.

```rust
let years: [i32; 3] = [1995, 2000, 2005];
```

This is an array of three `i32` elements, which is represented by the type
`[i32; 3]`. Notice that the length is part of the type - that's because for
arrays, the length is hardcoded at compile-time and never changes!

## Iteration

Unlike tuples, we can iterate over the elements in an array:

```rust
let years: [i32; 3] = [1995, 2000, 2005];

for year in years.iter() {
    println!("Year: {}", year);
}
```

This `.iter()` is a *method* on the `years` array. We'll learn about methods
later on.

## Writing elements

We can change individual elements in a `mut` array:

```
let mut years = [1995, 2000, 2005];

years[0] = 1996;
years[1] = 2002;
years[2] = 2007;
```

However, we cannot add or remove elements; an array's length is hardcoded.

## Reading elements

We can read elements using a similar syntax:

```
let years = [1995, 2000, 2005];

let first_year = years[0];
let second_year = years[1];
```

Unlike with tuples or structs, with arrays you can put variables in between the
brackets - for example, `years[x]`. If you do this, and the variable is outside
the bounds of the array, then the result will be a panic.

## Destructuring

Similarly to structs and tuples, we can take apart an array using destructuring:

```rust
let [year1, year2, year3] = years;
```

Since the length of the array is fixed at compile time, destructuring elements
from an array can never go out of bounds at runtime; the compiler would have
given an error if that were going to be a problem.
