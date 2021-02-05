# Enums

Enums, short for *enumerations*, define values that can be one of several
distinct alternative *variants* at runtime.

```rust
enum Color {
    Green,
    Yellow,
    Red,
}

let green: Color = Color::Green;
let yellow: Color = Color::Yellow;
let red: Color = Color::Red;
```

All three of these values have the type `Color`.

## Runtime representation

This `Color` enum will compile to a `u8` behind the scenes. One of the variants
will be assigned to the number 0, another to 1, and another to 2. If `Color`
had more than 256 variants, it would not fit in a `u8`, and Rust would quietly
upgrade to the next-largest unsigned integer (`u16`) behind the scenes.

By default, the first variant defined in the enum gets assigned 0, and then
each subsequent variant gets one higher than the previous one. You can check
this by using `as` to convert a variant to a number.

```
println!("In memory, Red is {}", Color::Red as u8);
```

If you want the variants to start on a number other than 0, you can assign the
first one to a different starting number. The others will increment as normal.

```rust
enum Color {
    Green = 1,
    Yellow,
    Red,
}
```

You can use `=` with more than one variant, including specifying different
backing numbers for every variant.

## `match` conditionals

We can make a conditional which checks which variant a value is.

```rust
let current_color:Color = Color::Yellow;

match current_color {
    Color::Green => {
        println!("green");
    }
    Color::Yellow => {
        println!("yellow");
    }
}
```

This conditional does nothing if `current_color` is `Color::Red`.

> `match` is similar to `switch` in some languages, but it only ever runs a
> single branch. You don't need to specify `break` to prevent fall-through;
> that happens automatically.

## Assigning values to `match` conditionals

Just like with `if,` we can use the output of a `match` directly as a value:

```rust
let color_str = match current_color {
    Color::Green => "green",
    Color::Yellow => "yellow",
    Color::Red => "red",
};
```

When we do this with `if`, we must have an `else` so there is always an explicit
value being assigned. Similarly, when we do this with `match`, we have to
cover all the variants in the enum. If we don't, we'll get a compile error.

We can use `_ =>` to get the equivalent of `else` - it will match all the
other variants.

```rust
let color_str = match current_color {
    Color::Green => "green",
    _ => "other"
};
```

## Matching numbers

You can also match on number literals (and string literals), in addition
to enum variants.

```rust
let color_str = match some_number {
    0 => "green",
    1 => "yellow",
    2 => "red",
    _ => "other",
};
```
