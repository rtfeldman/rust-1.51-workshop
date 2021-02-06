# Mutable References

Owned values can be mutated, and immutable references to those values cannot.

However, there is a type of reference which does allow mutation: a 
*mutable reference*. We can get one of those like so:

```rust
let mutable_years: &mut Vec<i64> = &mut years;

mutable_years.clear(); // clear() removes all elements from the Vec
```

The type of `mutable_years` is `&mut Vec<i64>`.

Note that this is not a `let mut`, but rather a plain old `let`. Because of 
that, we cannot reassign `mutable_years` to something else - that is, we can't
write `mutable_years = ...` or else we'll get a compile error - but since
`mutable_years` is itself a mutable reference, we can pass it to functions
which expect `&mut Vec<i64>` values.

As it happens, the `.clear()` method on `Vec` expects a mutable reference.
It's defined like so:

```rust
fn clear(&mut self)
```

Mutable references are strictly more powerful than immutable references, in
that you can pass a mutable reference to any function which expects an immutable
one, but not the other way around.

For example, you can call `.clear()` on a `&mut Vec<i64>`, and you can also
call `.len()` on it...but if you have a `&Vec<i64>`, you can call `.len()`
in it but not `.clear()`.

## Mutable reference restrictions

One downside of mutable references compared to immutable ones is that as long as
you have a mutable borrow active on a value, you are not allowed to have any 
other borrows (mutable or immutable) active on that value.

For example, this is totally allowed:

```rust
let years = vec![1995, 2000, 2005];

let years_ref1 = &years;
let years_ref2 = &years;
```

...however, this would not compile:

```rust
let years = vec![1995, 2000, 2005];

let years_ref1 = &mut years;
let years_ref2 = &mut years; // ERROR: tried to borrow while a mutable borrow was active!
```

...and neither would this:

```rust
let years = vec![1995, 2000, 2005];

let years_ref1 = &mut years;
let years_ref2 = &years; // ERROR: tried to borrow while a mutable borrow was active!
```

In practice, this restriction rarely comes up except when doing concurrency
in Rust - and when doing concurrency, it's very important.

We won't get into Rust concurrency in this workshop, but briefly: there's a
category of errors called "data races" where two different threads have access
to mutate a value at the same time, and the result can be really nasty bugs.
The "only one at a time" restriction means that Rust can rule out data races
at compile time. This is one of Rust's major selling points to anyone who
wants to write high-performance concurrent systems, since avoiding data races is
critically important in that domain.
