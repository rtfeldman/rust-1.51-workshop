# Strings

## Hello, World!

Here's a Rust program which prints "Hello, World!" to the console:

```rust
fn main() {
    println!("Hello World!");
}
```

If you put this in a file called `app.rs` and compile it by running `rustc app.rs`, it will generate a binary executable called `app` which you can then run to see the output.

## Defining variables with `let`

All variables in Rust are defined using `let`. This program will still print "Hello, World!" to the console:

```rust
fn main() {
    let greeting = "Hello";
    let subject = "World";

    println!("{}, {}!", greeting, subject);
}
```

This is because `println!` replaces the first `{}` with `greeting` and the second `{}` with `subject`. We can have as many or as few of these as we like.

> You can define a name using `let` without initializing it right away - for
> example, writing `let greeting;` and then later on `greeting = "Hello";` - but
> you must initialize the value before it's used. If there's any code path where 
> it could be used before it's initialized, Rust will give a compiler error.

## `format!`

`format!` works about the same way as `println!` - except instead of printing a
line to the console, it returns the string after making any `{}` substitutions.

```rust
let formatted_greeting = format!("{}, {}!", greeting, subject);
```

## Crashing with `panic!`

We can intentionally crash a Rust program using `panic!`. 
It supports the same `{}` interpolation syntax that `println!` does.

```rust
fn main() {
    let crash_reason = "Server wanted a nap.";

    panic!("I crashed! {}", crash_reason);

    println!("This will never get run.");
}
```

Rust does not have `try`/`catch`, so whenever you `panic!` you can really expect the program to crash. (We'll talk about how Rust does error handling and recovery later.)
