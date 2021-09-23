## Lifetime Elision

Rust requires that all struct and enum definitions include a lifetime
parameter with each reference they contain. So you could not write this:

```rust
struct Releases {
    years: &[i64],
    eighties: &[i64],
    nineties: &[i64],
}
```

...because the compiler would give an error saying that each of those fields
needed something like a `&'a [i64]` or `&'b [i64]` instead of `&[i64]`.

However, lifetime annotations like this are not always necessary. Sometimes,
the Rust compiler can figure out what the particular lifetimes must be, and
lets you write a plain `&`.

For example, lifetime annotations are not needed when there is exactly 1
reference in the return type and also exactly 1 reference in the arguments. In
that case, Rust knows the only way the function could be valid is if they have
the same lifetime, so it automatically gives them the same lifetime for you.

So instead of writing this:

```rust
fn foo<'a>(arg: &'a String) -> &'a i64
```

...you could write this:

```rust
fn foo(arg: &String) -> &i64
```

The Rust compiler would effectively rewrite it into the previous example behind
the scenes, inserting the `'a` parameters in the appropriate places, because
they would be necessary.

Lifetime elision is why functions like `len` don't need explicit lifetime
parameters:

```rust
fn len(&self) -> usize
```

The lifetime is still there, you just don't have to annotate it explicitly.
