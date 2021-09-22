# Borrowing

Let's revisit our `print_years` example from the previous section on ownership:

```rust
fn print_years(years: Vec<i64>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }

    // dealloc `years`
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    print_years(years); // transfer ownership of `years` to `print_years`
    print_years(years); // transfer ownership again!

    // Do NOT dealloc `years`, because its ownership was transferred elsewhere
}
```

This version of the code did not compile; it got a use-after-move error on the 
second call to `print_years` because the scope was attempting to transfer 
ownership of a value that it no longer owned.

We saw two ways to fix this:

1. Use `print_years(years.clone());` to transfer ownership of a clone instead of the original
1. Have `print_years` return the `Vec` it receives, so it transfers ownership back to the caller

We can also fix this by adding three `&` signs to the code in just the right
places, like so:

```rust
fn print_years(years: &Vec<i64>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    print_years(&years); // temporarily give `print_years` access to `years`
    print_years(&years); // temporarily give `print_years` access to `years`

    // dealloc `years`
}
```

This code will compile and work, yet we neither cloned `years` nor returned it
from `print_years`. How is this possible?

From an ownership perspective, this code is doing something similar to the
version of the code which had `print_years` return a `Vec`. In that version,
`print_years` temporarily had ownership over `years`; it received ownership
when `main` passed `years` it as an argument, but then it returned ownership of
that value back to `main` when it did `return years;` at the end.

In this version, the `&Vec<i64>` argument serves the same purpose as the 
previous approach of taking a `Vec<i64>` argument and then returning that same
`Vec<i64>` as the function's return value.

## References

A `&Vec<i64>` is a *reference* type - more specifically, a reference to 
a `Vec<i64>` value.

Having a reference to a value is different from owning that value.

* When a scope ends, all values owned by that scope either get moved to other scopes or dropped. (That is, deallocated.)
* References never cause anything to get dropped. 

However, you can't do as much with references as you can owned values.
* If a function requires being passed an owned value (e.g. `Vec<i64>` instead of `&Vec<i64>`), and all you have is a reference, you can't pass that reference in. You need to get an actual owned value.
* If you want to mutate a value, and all you have is one of these immutable references to it, you're out of luck. There's no way to do it. The Rust compiler is extremely strict about this, and there aren't any workarounds or "escape hatches" like telling the compiler "I know what I'm doing, just please let me use this immutable reference to mutate something." It can't be done!

Generally speaking, if a function can accomplish its goals by accepting a 
reference instead of an owned value, it's better to accept the reference.
The trade-off is that it means the function is restricted in what it can do,
but the caller is less restricted - so if the function can do everything it
needs to even with those restrictions, might as well give callers more
flexibility!

As we can see in our example here, one quality-of-life improvement this gives to
callers is that they no longer have to clone the argument when passing it in,
or do the dance of passing it in, accepting it as a return value, and then
using only the returned value from then on.

Methods can also accept references to `self` - for example, the `Vec` method
`.len()`, which returns the length of the vector, is defined like this:

```rust
fn len(&self) -> usize
```

with references and methods you don't need to borrow with &

This means you can call `.len()` on a vector to obtain its length without 
needing to clone the vector, or have `.len()` return the vector itself as well.

## Borrowing

*Borrowing* is how you obtain a reference from an owned value. You do it by
adding a `&` in front of the value. So if I have a `Vec<i64>` named `years`,
then I can borrow `years` by writing `&years` - giving me a `&Vec<i64>` 
reference to the `years` value.

The idea is that, much like when you let someone borrow something you own, the
borrowing is a temporary state of affairs; there's an understanding that you
still own the thing, and that whoever you lent it to will give it back to you
later. (Unlike in real life, in Rust you can lend out the same value more than 
once at the same time, but let's not dwell on the metaphor too much.)

In Rust, the length of a "borrow" is also based on scopes. When I pass my 
`&years` reference to `print_years`, I'm granting that function temporary access
to `years` (with the restriction that `print_years` can't move or mutate it).
Once `print_years` returns, that reference is no longer in any scope, and there 
are no longer any "active borrows" on my owned `years` value.

One way in which the "borrowing" metaphor is important is that as long as there 
is an "active borrow" on a value you own - that is, some scope somewhere has a 
reference to it - that value must not be dropped. If it were dropped, we'd have 
a use-after-free on our hands, where the scope that still has a reference to 
that value might try to look at it after it had been deallocated.

> This mainly comes up when doing asynchronous programming, and it can lead to a
> compiler error of "reference dropped while still borrowed" - but we won't get
> into asynchronous programming in this workshop.

Incidentally, Rust's compiler errors around ownership and borrowing (such as
the "use after move" error we saw earlier) are collectively known as "the
borrow checker," because they surface a category of errors that are separate
from more common compiler errors like type errors and naming errors.

## Turning off the Borrow Checker

Rust core team member Steve Klabnik wrote an article entitled [You can't "turn off the borrow checker" in Rust](https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust) 
because there's a common misconception that this is possible using
the `unsafe` keyword. We won't get into the `unsafe` keyword in this workshop
because it's for doing things like calling out to C code, but contrary to the 
misconception, it doesn't have any effect on the borrow checker.

There is, in fact, no way to turn off the borrow checker!
