# Ownership

Let's go back to our previous example of the function which returns a `Vec`.

```rust
fn get_years() -> Vec<i32> {
    let years = vec![1995, 2000, 2005, 2010, 2015];

    return years;
}

fn main() {
    let all_years = get_years();
}
```

Here, we once again have a `Vec` that gets created in the middle of a function.
However, this time it gets returned - meaning we do *not* want Rust to call
`dealloc` on it when it goes out of scope. If that happened, then when `main`
tried to use it, that would be a use-after-free!

## Ownership and moving

This is where Rust's concept of "ownership" saves the day. In Rust terminology,
every value is "owned" by a particular scope. At first, it's owned by the scope
where it was originally created, but ownership can be passed to other scopes
later on. For example, when we call `return years`, that transfers ownership of 
the `years` vector from the `get_years` function's scope to its caller's scope.

Transferring ownership of a value is called "moving" that value in Rust. This
term comes up in Rust compiler error messages, so it's a good one to know!

*Moving* a value (from one scope to another) doesn't cause it to get 
deallocated. That's because if the value is still in-scope somewhere, that means 
there's still code which could potentially read that memory...and therefore it's 
not safe to deallocate without potentially causing a use-after-free.

Instead, deallocation gets triggered when there is no longer *any* scope 
owning a value, which means the value went out of scope without having been 
moved to a different scope first.

With those rules in mind, here's how allocations and deallocations happen for
teh `get_years` example:

```rust
fn get_years() -> Vec<i32> {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    return years; // transfer ownership of `years` to caller's scope ("move")
}

fn main() {
    let all_years = get_years(); // take ownership of returned value (`years`)

    // dealloc `all_years`
}
```

At the very end, when `all_years` goes out of scope and `dealloc` gets called at
the end of `main`, it will be deallocating the memory that was originally
allocated in `get_years`.

## Ownership of arguments

When a function receives an argument like `years: Vec<i64>`, it will take
ownership of that value and deallocate it when the function's scope ends:

```rust
fn print_years(years: Vec<i64>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }

    // dealloc `years`
}
```

When something calls `print_years`, it's essentially saying "I am moving this
value to your scope, so you are now responsible for deallocating it when
you're done with it."

In contrast, if `print_years` were to return the `Vec` when it was done, then
no deallocation would happen here:

```rust
fn print_years(years: Vec<i64>) -> Vec<i64> {
    for year in years.iter() {
        println!("Year: {}", year);
    }

    // no dealloc - instead, move it to the caller's scope!
    return years;
}
```

Because it affects deallocation, whether or not a function returns a value it
owns is a much bigger deal in Rust than it is in most other languages.

For example, let's say we had the version of `print_years` that didn't return
the `Vec` it was passed, and then we wrote this code:

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

If this code successfully compiled and ran, it would cause a use-after-free!

At the end of `print_years`, the `years` vector is automatically deallocated,
which means that when we try to call `print_years` on it again (which will 
attempt to run a `for` loop over it), we will be looping over deallocated 
memory. That's a use-after-free!

## use-after-move errors

Fortunately, Rust will prevent this use-after-free by giving us a compile error.
At one point we tried "transfer ownership again," which the Rust compiler
does not allow. Once a value has been moved to a new scope, the original scope 
no longer owns it, and can no longer reference it anymore!

As of Rust 1.49, the compile error looks like this:

```
error[E0382]: use of moved value: `years`
  --> src/main.rs:13:17
   |
10 |     let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`
   |         ----- move occurs because `years` has type `Vec<i64>`, which does not implement the `Copy` trait
11 | 
12 |     print_years(years); // transfer ownership of `years` to `print_years`
   |                 ----- value moved here
13 |     print_years(years); // transfer ownership again!
   |                 ^^^^^ value used here after move
```

It's very common for Rust beginners to encounter "use of moved value" errors 
like this. (It's less common for experts, but still comes up quite a bit!)

This is telling us that we've attempted to use a value after it was moved.
If use-after-move were allowed, it would result in a use-after-free, and Rust
prevents that use-after-free from happening with compiler errors like this.

## Fixing a use-after-move error

We can fix this by having `print_years` return the `Vec` it gets passed,
which means it will transfer ownership of that `Vec` back to the caller.

```rust
fn print_years(years: Vec<i64>) -> Vec<i64> {
    for year in years.iter() {
        println!("Year: {}", year);
    }

    return years; // transfer ownership of `years` to the caller
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    let years2 = print_years(years); // transfer ownership of `years` to `print_years`
    let years3 = print_years(years2); // transfer ownership of `years2` to `print_years`

    // Do NOT dealloc `years`, because its ownership was transferred elsewhere
    // Do NOT dealloc `years2`, because its ownership was transferred elsewhere

    // dealloc `years3`
}
```

This code will compile and work just fine. 

> Note that if we changed `for year in years2` to be `for year in years`, 
> it would once again get a compile error of use-after-move!

## `.clone()`

If we didn't want `print_years` to return a `Vec`, another way we could have
prevented the earlier use-after-move error would have been to call `years.clone()`
in order to pass an identical clone of `years` to `print_years`, instead of the
original:

```rust
fn print_years(years: Vec<i64>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }

    // dealloc `years`
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    print_years(years.clone()); // transfer ownership of a *clone* of `years` to `print_years`.
                                // (`print_years` will deallocate this clone.)

    print_years(years); // transfer ownership of `years` to `print_years`

    // Do NOT dealloc `years`, because its ownership was transferred elsewhere
}
```

The `clone` method reconstructs the `Vector` in a new memory location, using
fresh calls to `alloc` as necessary. This approach is quicker to implement than
the fix we applied where we returned `Vec`, but it comes at the cost of runtime
performance. Cloning takes much longer than returning!

That said, when you're starting out, it's fine to use `.clone()` to get past
use-after-move errors like this. Yes, it comes at a performance cost, but the
cloning is explicit, so you can always go back later and optimize it away once
you've gotten the program running!

## Converting to `mut`

Ownership means it's safe to convert an owned immutable value to a mutable one,
without sacrificing immutable semantics in any meaningful way.

```rust
fn organize_years(years: Vec<i64>) {
    let mut mutable_years = years; // Converting immutable to mutable!

    mutable_years.pop();
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    organize_years(years);

    // The compiler won't let us reference years past this point.
}
```

In this example, the `organize_years` function takes a `Vec` which is declared
as immutable, yet it ends up mutating it!

This is semantically safe, because the `years` parameter is owned. That means
the caller won't be able to reference this value anymore after passing it in,
which in turn means the caller won't be able to tell whether it gets mutated
after that point. Since mutating it couldn't possibly affect anything, it is
allowed even though the parameter was declared as immutable.

This allows for better performance by reusing existing memory rather than 
having to clone a value that was on its way to being deallocated anyway.
