# Methods

## `impl` for namespacing

We saw earlier how enum variants are namespaced by the name of the enum.
For example, the `Color::` in `Color::Green` here:

```rust
enum Color {
    Red,
    Yellow,
    Green,
}

let green = Color::Green;
```

The `impl` keyword lets us use this namespacing for custom purposes, like
our own functions:

```
impl Color {
    fn is_red(color: Color) -> bool {
        match color {
            Color::Red => true,
            _ => false,
        }
    }

    fn num_colors() -> usize {
        3
    }
}
```

Now we can call `Color::is_red(Color::Yellow)` to get back `false`.
We can also call `Color::num_colors()`, which will always return 3.

> `impl` isn't limited to being used on enum types; it also accepts struct 
> types, among others.

## `self` for methods

We can turn `is_red` into a *method* by changing its `color: Color` argument
to `self`:

```rust
impl Color {
    fn is_red(self) -> bool {
        match self {
            Color::Red => true,
            _ => false,
        }
    }
}
```

The `self` argument is a special keyword in Rust. It never has a type listed,
because its type is always the type that comes after the `impl` keyword (so
in this case, `Color`). Also, when it's used, it must always be the first
argument to a function, and the function can have at most one `self` argument.

Changing from `color: Color` to `self` doesn't break any of our existing code.
We can still call `Color::is_red(Color::Yellow)` like before. However, now
we can optionally call it using *method-calling syntax* like so:

```rust
let yellow = Color::Yellow;
let is_yellow_red = yellow.is_red();
```

> We saw this syntax in the previous section with `array.iter()`. 

This is all there is to methods in Rust. To make one, use the `self` keyword
as the first argument to a function. That unlocks the special calling syntax 
sugar, and nothing else.
