# Automatic memory management

## Automatic `alloc`

In Rust, unlike in C, calls to `alloc` and `dealloc` are done automatically by 
default. (You may notice we haven't had to call any functions by those names, 
even though we've been using `Vec` values.) When you create a new `Vec` using
the `vec!` macro, for example, that macro results in an `alloc` call.

## Automatic `dealloc`

When does Rust call `dealloc`? That's more involved. The short answer is that
the Rust needs to balance these goals:

1. It's better to `dealloc` memory sooner rather than later, to avoid memory leaks.
1. To prevent double-free, memory must only be deallocated once per `alloc`.
1. To prevent use-after-free, memory must not be deallocated if it still might be referenced anywhere.

Here's a basic example of how Rust automatically performs `alloc` and `dealloc`.

```rust
fn get_final_orders() -> i64 {
    let orders = vec![1, 2, 3, 4]; // `alloc` gets called to store the Vec
    let mut total_orders = 0;

    for order in orders.iter() {
        total_orders += orders;
    }

    let final_orders = total_orders * 2;

    // `dealloc` gets called on the memory index of the `orders` Vec
    return final_orders;
}
```

This function returns an `i64`. During the course of running the function, it
creates a `Vec` named `orders`, but that `Vec` doesn't get returned. This means
once the function returns, and `orders` has gone out of scope, it's no longer
possible for any other part of the program to access it. That means it's safe to
deallocate as soon as it's about to go out of scope, which is exactly what Rust
does here!

## Scopes and deallocation

In this example, we didn't quite deallocate `orders` as soon as we could have.
We deallocated it when it went out of scope, but it was no longer in used after
the `for` loop, so it would have been safe to deallocate before running that
`final_orders = total_orders * 2` logic.

In C, since we would be calling `dealloc` explicitly, we could choose to free
that memory sooner if we wanted to reclaim those resources sooner. Can we
achieve the same thing in Rust? Yes, by introducing a new *anonymous scope*:

```rust
fn get_final_orders() -> i64 {
    let mut total_orders = 0;

    {
        let orders = vec![1, 2, 3, 4]; // `alloc` gets called to store the Vec

        for order in orders {
            total_orders += orders;
        }

        // `dealloc` gets called on the memory index of the `orders` Vec
    }

    let final_orders = total_orders * 2;

    return final_orders;
}
```

The code inside the `{` and `}` here is inside an anonymous scope. Just like
how we get a new scope when we write a function, an `if`, or a `match`, we can
also create a scope out of thin air like this. One of the uses of doing so is
to control when things go out of scope, in order to trigger Rust's automatic
`dealloc` sooner if desired.

Here, by putting `orders` in a scope that ends before `let final_orders = ...`,
we caused `orders` to go out of scope - and thus get deallocated - before the 
`final_orders` logic ran.

In C, we would have had to write these calls to `alloc` and `dealloc` manually,
but Rust was able to figure out where they should go and make the calls for us, 
with no possibility of use-after-free or double-free. This is an example of
how Rust is able to achieve C-like speeds without C-like safety issues; it's
compiling to the same code an equivalent C program would have, but without as
much potential for programmer mistakes.
