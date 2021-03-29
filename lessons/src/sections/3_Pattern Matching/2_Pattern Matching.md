# Pattern matching

Enum variants can optionally include *payloads*.

```rust
enum Color {
    Green,
    Yellow,
    Red,
    Custom { red: u8, green: u8, blue: u8 }
}

let green: Color = Color::Green;
let yellow: Color = Color::Yellow;
let red: Color = Color::Red;
let blue: Color = Color::Custom { red: 0, green: 0, blue: 255 };
```

Here, the Red, Yellow, and Green variants work about the same way as before.
The Custom variant has a built-in struct with three `u8` fields. We must
specify those fields when constructing a `Color::Custom` variant, and we can
deconstruct them in a `match` to read their values.

```rust
match current_color {
    Color::Green => {
        println!("green");
    }
    Color::Yellow => {
        println!("yellow");
    }
    Color::Custom { red, green, blue } => {
        println!("custom (RGB {}, {}, {})", red, green, blue);
    }
}
```

This is known as *pattern matching*. Each of the conditions before a `=>`
arrow is called a *pattern*, and at runtime Rust will try checking each
pattern in turn to see which one matches. The first time it finds a match,
it will run the corresponding code and then exit the `match`.

## Matching multiple variants

You can match on a variant multiple times.

```rust
match current_color {
    Color::Green => {
        println!("green");
    }
    Color::Custom { red: 0, green, blue } => {
        println!("custom color with no red (RGB 0, {}, {})", green, blue);
    }
    Color::Custom { red, green, blue } => {
        println!("custom (RGB {}, {}, {})", red, green, blue);
    }
}
```

## Tuple payloads

If you don't want to name the fields in a payload, you can use a tuple payload:

```rust
enum Color {
    Green,
    Yellow,
    Red,
    Custom(u8, u8, u8)
}
```

In that case, when pattern matching on them, use a similar syntx to destructure
the tuple payload:

```rust
match current_color {
    Color::Green => {
        println!("green");
    }
    Color::Yellow => {
        println!("yellow");
    }
    Color::Red => {
        println!("red");
    }
    Color::Custom(red, green, blue) => {
        println!("custom (RGB {}, {}, {})", red, green, blue);
    }
}
```

## Runtime representation

As previously discussed, this enum would be a `u8` at runtime.

```rust
enum Color {
    Green,
    Yellow,
    Red,
    Custom
}
```

`Green` would be 0 at runtime, `Yellow` would be 1, `Red` would be 2, and
`Custom` would be 3. If we introduce a payload, things change:

```rust
enum Color {
    Green,
    Yellow,
    Red,
    Custom(u8, u8, u8)
}
```

Now, `Color::Custom` has a runtime representation of *four* `u8` values. The 
first `u8` holds the number 4, just like before. That's the value that `match` 
examines at runtime to be able to tell which variant this is, and we call that
value "thetdiscriminant." The other three `u8` values are its payload.

This means that a `Color::Custom` variant takes up 4 bytes at runtime: one byte
for the `u8` discriminant, and three more bytes for its `(u8, u8, u8)` payload.

A rule of enums is that each variant always takes up exactly the same amount 
of memory, no matter what's in it. This means that because `Color::Custom` takes
up 4 bytes at runtime, so do `Color::Green`, `Color::Yellow`, and `Color::Red`,
even though they don't have payloads. Those variants consist of a `u8` 
discriminant followed by three bytes of unused memory.

This is necessary because Rust has to know at compile time exactly how much
memory to allocate for every value. Consider this function:

```rust
fn color_to_string(color: Color, max_string_length: u32) -> String
````

Rust needs to know how much memory, to reserve for the `Color` argument, but
it can't know ahead of time what variant will go in there. This function could
be called with any `Color` variant. To support all the variants, it needs to
have room for the largest variant, so the largest variant ends up determining
the size of all the variants. (This is true in other situations too, not just
function arguments.)

This has implications for performance: increasing the size of the largest 
variant in an enum increases the size of all the others as well, but increasing 
the size of a variant that *isn't* the largest has no effect on the sizes of 
any other variants.
