# Floats

The `println!` macro also works with numbers.

```rust
fn main() {
    let x = 1.1;
    let y = 2.2;

    println!("x times y is {}", x * y);
}
```

This prints "x times y is 2.4200000000000004" rather than the of 2.42 we'd get
using normal arithmetic. That's because these are standard [IEEE-754 binary floating-point numbers](https://en.wikipedia.org/wiki/IEEE_754#Binary), which
is what many languages use to represent decimal numbers. Various operations on
these numbers can result in values of infinity, negative infinity, or 
"not a number" - when certain error conditions happen, like division by zero.

## Mutability

Once a `let` value has been initialized, it cannot be reassigned to a different
value. For that, you need `let mut`.

```rust
fn main() {
    let x = 1.1; // x can never be reassigned
    let mut y = 2.2;

    y = 3.3; // Works fine
    y += 4.4; // Syntax sugar for y = y + 4.4

    println!("x times y is {}", x * y);
}
```

If we added `x = 2.2;` to the end of this function, it would no longer compile.
That's because `x` was not defined with `let mut`! In general, Rust tends to
prefer giving errors at compile time rather than at runtime.

## Numeric types

Rust also gives *type errors* at compile time. Rust is a statically type-checked 
language, which means every value has a single type associated with it at 
compile time, and that type can never change at runtime.

For example, if we define `let mut y` and first assign it to a number, then
later a string, Rust will give us a compile-time error:

```rust
let mut y = 2.2;

y = 3.3; // no problem
y = "three point three"; // compile-time error; y changed types!
```

We can use *type annotations* to specify exactly which types our values have. 
For example, we could add type annotations to our `x` and `y` declarations:

```rust
fn main() {
    let x: f64 = 1.1;
    let y: f64 = 2.2;

    println!("x times y is {}", x * y);
}
```

This says that `x` and `y` both have the type `f64`, which is short for 
"64-bit floating-point number." 64-bit floating point numbers are also what
JavaScript uses for its normal number type.

With `let` this type annotation is optional by default, because Rust has a
*type inference* system that infers types for values automatically. If it's
unable to infer the type, it will give a compile-time error and ask you to
add an annotation to specify exactly the type you want, but it will never 
silently infer the wrong type.

## Functions

When we declare a function, the types of its arguments and return value are not 
inferred; we have to write the type annotations explicitly:

```rust
fn main() {
    println!("1.1 times 2.2 is {}", multiply_both(1.1, 2.2));
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}
```

This says that `multiply_both` takes two `f64` arguments and returns a `f64`.
The reason we don't have any type annotations on our `main` function is that it 
has no arguments and no return value.

Notice that `println!` ends in a `!` but `multiply_both` does not. The `!`
indicates that `println!` is a *macro*, not a *function*. Rust macros always end 
in `!` whereas normal function calls never do. Macros can do some things that
functions can't (such as the `{}` formatting trick), but they also have some
downsides compared to functions. You can define your own macros, but since
doing that is generally considered an advanced technique, we won't dive any
further into macros in this introductory workshop.

## Float sizes

Rust has two different sizes of float: `f64` and `f32`. The former is 64 bits
(8 bytes) large, whereas the latter is 32 bits (4 bytes) large.

The trade-off is that `f64` can store more digits, making it more precise and
able to represent larger numbers, but it takes up more memory. This memory
difference can really add up in applications that store massive quantities of 
floating-point numbers. For example, in 3D games, `f32` is very commonly used
instead of `f64`.

## Converting between numeric types with `as`

`f32` and `f64` are different types, so if you try to use them interchangeably,
you'll get a type mismatch at compile time:

```rust
fn main() {
    let x: f64 = 1.1;
    let y: f32 = 2.2;
    let z = x * y;

    println!("x times y is {}", z); // ERROR: incompatible types!
}
```

You can use the `as` operator to convert one numeric type to another.

```rust
fn main() {
    let x: f64 = 1.1;
    let y: f32 = 2.2;

    println!("x times y is {}", x * y as f64);
}
```

Converting from `f32` to `f64` just takes up a bit more memory, but converting
from `f64` to `f32` can result in information loss because `f32` can't store
as much information. This means converting from `f64` to `f32` and back to
`f64` can result in a different `f64` value than the one you started out with!

> You can only use `as` to convert between numeric types, not to
> (for example) convert between strings and numbers.
