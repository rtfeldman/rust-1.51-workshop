# Structs

Structs are syntax sugar for tuples. They are identical at runtime, so there
are no performance tradeoffs. The difference is at compile time: struct
elements are referenced by name rather than by position.

```rust
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn point(x: i64, y: i64, z: i64) -> Point {
    Point { x: x, y: y, z: z }
}

fn get_x(point: Point) -> i64 {
    point.x
}

fn set_x(mut point: Point, x: i64) {
    point.x = x;
}
```

`Point { x, y, z }` is optional syntax sugar for `Point { x: x, y: y, z: z }`.

## Destructuring

You can use *destructuring* syntax to take apart a struct into one or more of 
its fields.

```rust
fn x_plus_y(Point { x, y }) -> i64 {
    x + y
}
```

If you don't want to name all the fields, you can write `..` in place of the
ones you don't want.

```rust
fn get_x(Point { x, .. }) -> i64 {
    x
}
```

You can also use this syntax in a `let` - e.g. `let Point { x, .. } = point;`
