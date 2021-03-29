## The Static Lifetime

There is a special lifetime called `'static`. This is a reserved word, and you
can never choose `'static` as the name of one of your lifetime parameters.

The static lifetime refers to values that live for the entire duration of
the program. They are never allocated, and they're never deallocated. Rather
than residing in the stack or the heap, these reside in the program's
binary itself.

One commen area where this lifetime comes up is in string literals. All
string literals in Rust have the type `&'static str`.

```rust
let name: &'static str = "Richard";
```

This is because Rust, like C and several other languages, stores its string
literals in the program's binary as a performance optimization. After all,
the bytes needed for those strings must be stored in the binary anyway -
otherwise, how would the binary know what to put on the heap when it's
running? - and so it can save heap allocations and deallocations by representing
the string literals in the program as references into those bytes in the binary.
