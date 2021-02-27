# Tuples, structs, and arrays in memory

At runtime, tuples, structs, and arrays all have the same representation:
they are sequences of adjacent bytes in memory, with no additional overhead.

For example, here are three values which have exactly the same representation
in memory:

```rust
struct Point { x: i64, y: i64, z: i64 }

let point_struct: Point           = Point { x: 0, y: 4, z: 2 };
let point_tuple:  (i64, i64, i64) = (0, 4, 2);
let point_array:  [i64; 3]        = [0, 4, 2];
```

Each of these values takes up 24 bytes of memory:

* The first 8 bytes are where the first `i64` is stored (`x` in the `struct`)
* The next 8 bytes are where the second `i64` is stored (`y` in the `struct`)
* The last 8 bytes are where the last `i64` is stored (`z` in the `struct`)

# Accessing values

When Rust compiles tuple-related logic into machine code, it translates field
names into byte offsets based on their types. For example, `.0` gets compiled to 
"read 8 bytes from the beginning of the tuple" whereas `.2` gets compiled 
to "read 8 bytes starting 16 bytes after the beginning of the tuple." Indexing
into arrays works the same way.

Structs work similarly, except there's an additional step: first the `.x` or
`.y` or `.z` is translated into the tuple equivalent of either `.0`, `.1`, or 
`.2`. Which one it translates to depends on the order in which the fields are
listed in the `struct` definition. Since `x` was listed first in `struct Point`,
`x` corresponds to `.0`.

> The reason Rust requires an explicit `struct` definition, while tuples can
> be defined anonymously, is that it needs an ordering so it can know which 
> indices the fields should map to.

# Performance

Tuples, structs, and arrays all have exactly the same performance 
characteristics. It's no more efficient to choose any one over any other,
because they all have the same runtime representation, and all of their
read and write operations compile to the same machine code.

Choose between them based on which fits your semantics best!
