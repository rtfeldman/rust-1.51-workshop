# Lifetime Annotations

Lifetime annotations are a way to track *dependencies* between lifetimes.

Here's a way to expand the example from the previous section to include lifetime annotations:

```rust
struct Releases<'a> {
    years: &'a [i64],
    eighties: &'a [i64],
    nineties: &'a [i64],
}

fn jazz_releases<'a>(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];

    Releases {
        years,
        eighties,
        nineties,
    }
}

fn main() {
    let releases = {
        let all_years: Vec<i64> = 
            // alloc
            vec![
              1980, 1985, 1990, 1995, 2000, 2000
            ];

        jazz_releases(&all_years)
    }; // dealloc

    for year in releases.eighties.iter() {
        println!("Eighties year: {}", year);
    }
}
```

Note that `'a` has appeared in several new places. This is a *lifetime parameter*,
and it works like a type parameter for lifetimes. (Like type parameters, you 
can pick any name you want instead of `a`, such as `'b`, `'foo`, `'blah`, etc.)

The `Releases` struct now says that each of its slices all have the same
lifetime:

```rust
struct Releases<'a> {
    years: &'a [i64],
    eighties: &'a [i64],
    nineties: &'a [i64],
}
```

We also could have said they all have different lifetimes, like so:

```rust
struct Releases<'a, 'b, 'c> {
    years: &'a [i64],
    eighties: &'b [i64],
    nineties: &'c [i64],
}
```

...However, in our code example, all three lifetime parameters would end up with
the same lifetime anyway. That's because inside the `jazz_releases` function,
we assign three slices to `Releases`, and they are all slices from the same
`Vec`. That means they all must have the same lifetime.
