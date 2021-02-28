# Tuples

A tuple is a collection of two or more values. Here is a `point` tuple 
containing three values:

```rust
let point: (i64, i64, i64) = (0, 0, 0);

fn get_x(my_point: (i64, i64, i64)) -> i64 {
    my_point.0
}

fn get_y(my_point: (i64, i64, i64)) -> i64 {
    my_point.1
}

fn get_z(my_point: (i64, i64, i64)) -> i64 {
    my_point.2
}
```

You can't dynamically change the type of a tuple at runtime. That means both
that the number of elements can't change at runtime, and also that their
types can't change. 

You can change the values inside a tuple as long as it's declared as `mut`:

```rust
fn set_x(mut point: (i64, i64, i64), x: i64) {
    my_point.0 = x;
}

fn set_y(mut point: (i64, i64, i64), y: i64) {
    my_point.1 = y;
}

fn set_z(mut point: (i64, i64, i64), z: i64) {
    my_point.2 = z;
}
```

If the tuple is not declared as `mut`, then it is immutable.

## Destructuring

You can use *destructuring* syntax to disassemble a tuple into its elements 
without naming the tuple as a whole.

```rust
fn add_all((x, y, z): (i64, i64, i64)) -> i64 {
    x + y + z
}
```

If you don't want to name all the elements, you can write `_` in place of each
field you don't want.

```rust
fn x_plus_y((x, y, _): (i64, i64, i64)) -> i64 {
    x + y
}

fn x_plus_x((x, _, _): (i64, i64, i64)) -> i64 {
    x + x
}
```

You can also use this syntax in a `let` - e.g. `let (x, y, _) = point;`

## Unit

There's no such thing as a tuple with one element in it, but there is a
tuple with zero elements in it. Its type is `()` (pronounced "unit") and its 
value is also `()`.

```rust
let unit: () = ();
```

Since `()` can't hold any information whatsoever, it's commonly used as a return
value for functions that perform an effect but have nothing interesting to
return. For example, the `println!` macro returns `()`.

```rust
let println_return_val: () = println!("Hello, World!");
```

Some languages have functions return "void" or have "no return value," but in
Rust, every function has a return value. If the function has nothing useful to
return, by convention it should return `()`.
