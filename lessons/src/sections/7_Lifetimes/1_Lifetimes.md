# Lifetimes

A *lifetime* is the time between when a value is allocated and when it's
deallocated.

Here's an example:

```rust
fn jazz_releases(years: &[i64]) -> Releases {
    let eighties: &[i64] = &years[0..2];
    let nineties: &[i64] = &years[2..4];

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

        jazz_releases(all_years)
    }; // dealloc

    for year in releases.eighties.iter() {
        println!("Eighties year: {}", year);
    }
}
```

Here the lifetime of `all_years` begins at the `// alloc` comment, when `vec!`
gets called, and ends at the `// dealloc` comment, when `all_years` goes
out of scope.

One of the rules Rust's borrow checker enforces is that you can never reference
a value after its lifetime has ended. (If you did, it would be a use-after-free
bug.)

Sometimes these borrow checker errors can come as a surprise. For example, in
the code above, the call to `println!` references `releases.eighties`, but
`releses.eighties` is a slice of `all_years` elements - and `all_years` has
already been deallocated by the time the `println!` call is reached.

Another way of saying this is that the *lifetime* of `releases` is tied to
the lifetime of `all_years`. When the lifetime of `all_years` ends, the lifetime
of `releases` also ends. In this case, that means the lifetime of `releases`
is never valid, because `all_years` goes out of scope before `releases` even
receives its value! That's what leads to the borrow checker error.

The more references a program has, the trickier it can get to follow which
references' lifetimes depend on which others. Fortunately, Rust has a way to
keep track of them explicitly.
