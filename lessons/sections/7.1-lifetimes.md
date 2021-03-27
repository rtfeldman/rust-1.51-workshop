# Lifetime Annotations

lifetimes are all about tracking *dependencies*

## Slices

struct VecMetadata {
    memory_index_of_first_element: usize,
    length: usize,
    capacity: usize,
}

struct SliceMetadata {
    memory_index_of_first_element: usize,
    length: usize,
}

slices don't get allocated on the heap
rather, they reference an existing heap
allocation this can be a vec

e.g.

```rust
let years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2005, 2010];
let eighties: &[i64]  = &years[0..2];
let nineties: &[i64]  = &years[2..4];

println!("We have {} years in the nineties", nineties.len());
```

## References in structs

```rust
struct Releases {
    years: &[i64],
    eighties: &[i64],
    nineties: &[i64],
}

fn get_pop_releases(years: &[i64]) -> Releases {
    let eighties: &[i64] = &years[0..2];
    let nineties: &[i64] = &years[2..4];

    Releases {
        years,
        eighties,
        nineties,
    }
}

let releases = {
    let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2005, 2010, 2015];

    get_pop_releases(all_years)
};

let eighties = releases.eighties; // uh oh!
```

```rust
struct Releases<'a> {
    years: &[i64],
    eighties: &'a [i64],
    nineties: &'a [i64],
}

fn get_pop_releases(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];

    Releases {
        years: recent_years,
        eighties,
        nineties,
    }
}

let releases = {
    let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2005, 2010, 2015];

    get_pop_releases(all_years)
};

let eighties = releases.eighties;
```

Lifetime annotations are often needed when *returning* references.

## The `'static` Lifetime

let name: &'static str = "Richard";


## Lifetime Elision

Lifetime annotations are not needed when there is exactly 1 reference in
the return type and also exactly 1 reference in the arguments. In that case,
Rust knows the only way the function could be valid is if they have the same
lifetime, so it automatically gives them the same lifetime for you.

## TODO example of 'b
