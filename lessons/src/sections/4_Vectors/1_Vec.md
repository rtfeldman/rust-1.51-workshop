# Vec

A vector, or `Vec` for short, is like an array whose size can change at runtime.

We can add an element to the end of the vector using its `.push()` method. We 
can get its current length using its `.len()` method.  We can create a `Vec` 
using the `vec!` macro, like so:

```rust
let mut years: Vec<i32> = vec![1995, 2000, 2005];

years.push(2010); // Now `years` has 4 elements in it, ending in 2010
years.push(2015); // Now `years` has 5 elements in it, ending in 2015

println!("Number of years: {}", years.len());
```

Like arrays, every element in a `Vec` must have the same type. Unlike arrays,
the length of a `Vec` can vary at runtime. For this reason, arrays have their
length in their type, but vectors do not. Note that the syntax for the element
type is a bit different between arrays and vectors - it uses angle brackets
instead of square braces:

```rust
let years_array: [i32; 3] = [1995, 2000, 2005];
let years_vec: Vec<i32> = vec![1995, 2000, 2005];
```

## Runtime Representation

The runtime representation of vectors is very different from arrays.
For example, let's say we have a `Vec` of three `i32` values and compare that
to an array of three `i32` values.

As we saw before, an array of three `i32` values has the same runtime 
representation as a tuple of `(i32, i32, i32)` - which is 12 bytes total, since
each `i32` takes up 4 bytes in memory.

In contrast, the `Vec` takes up memory in two different locations. First it
has the 12 bytes that stores the actual `i32` values, but then it has an 
additional chunk of memory needed to store metadata about them. This extra
metadata is what allows the `Vec` to grow at runtime, and it looks something
like this:

```rust
struct VecMetadata {
    memory_index_of_first_element: usize,
    length: usize,
    capacity: usize,
}
```

> `usize` is equivalent to `u32` (4 bytes) on a 32-bit system, and `u64` (8
> bytes) on a 64-bit system.
>
> Since most systems these days are 64 bits, this means that a `Vec` of three
> `i32` values will take up a total of 36 bytes in memory: 24 bytes of metadata,
> and then 12 bytes of actual data!

Let's take a look at what that metadata is actually doing.

## Memory Index

A useful way to think of memory is that it's a giant array of bytes. 

With that mental model, the type of 4 gigabytes of memory would be
`let mut memory_bytes: [u8; 4_000_000_000]`. We can read any of those bytes, and,
since it's `mut`, we can also write any of them...but we can't change the length
of the array, since the total amount of memory available to a process is 
dictated by the operating system and the hardware it runs on.

The `memory_index_of_first_element` and `length` fields in our `VecMetadata` 
struct have enough information to describe which of these bytes in `memory`
are storing our elements. (We'll see how `capacity` fits in later.)

Let's say we were storing our three `i32` values at memory index 100 - that is,
`memory_bytes[100]` would store the first byte of the first `i32`. Since three
`i32` values take up a total of 12 bytes, we'd need to use `memory_bytes[100]`
through `memory_bytes[111]` to store all three of them side by side. In this
example, `memory_index_of_first_element` would be set to `100`, because that's
the index in memory where the first byte of the first element lives.

## Length

Say we want to iterate over the elements in this `Vec`. If all we had was the
index of the first byte, how would we know how many elements to read? We could
start by reading the first `i32` at that index...but what if the `Vec` is empty?
In that case, we shouldn't even read one element!

The `length` field makes iteration possible. It tells us how many elements
are *currently* in the `Vec`, which is the exact number of iterations a `for` 
loop should perform. The loop begins by reading an `i32` worth of bytes 
beginning at `memory_index_of_first_element` and then moves to the next `i32`
in the next iteration of the loop by advancing the index it's reading from
by 4 bytes (since an `i32` is 4 bytes).

## Changing length at runtime

Suppose we have two `Vec` instances storing their bytes back-to-back
in memory. So the first one begins at `memory_bytes[100]` and ends at 
`memory_bytes[111]`, and the next one begins at `memory_bytes[112]`.

Now let's say we want to `.push()` an element onto the first `Vec`. That will
increase `length` by 1, and store the new element in `memory_bytes[112]`.
This is a problem, though, because `memory_bytes[112]` is already in use by
the other `Vec`! If we write the new element into that slot in memory, we'll
be overwriting the other `Vec`'s first element, which would be really bad.

In this situation, `.push()` will avoid this problem by moving the entire
contents of the `Vec` to a new location in memory that has enough room. It will
do this by asking Rust's memory allocation bookkeeping system to find a memory
index which has enough unused bytes after it to accommodate the new `length`.
Then it copies all the bytes from the old location to the new location, adds the
new element at the end, and informs the allocation bookkeeping system that the
previously-used bytes are no longer in use.

At the end of this `.push()`, not only has the `Vec`'s `length` change, but
its `memory_index_of_first_element` has changed as well.

## Capacity

With this design so far, calling `.push()` several times in a row can 
potentially result in lots of moving bytes around. If the new memory index is 
also a "tight fit," then calling `.push()` again will result in another 
expensive memory relocation.

To avoid this expensive scenario, `Vec` requests some extra bytes at the end as
buffer. They aren't used to store elements at first, but they are guaranteed to 
be free memory slots, so they *can* be used to store future elements without
needing to relocate everything.

The `capacity` field stores the total number of elements that can be stored
before a relocation is needed. `capacity` is always greater than or equal to
`length`. Initially - after calling `vec!` to initialize the vector - `length`
and `capacity` are equal, so the first `push` will require a relocation.

However, after that first relocation happens, `push` will request twice as many
bytes as it strictly needs, to have a substantial buffer for future `push`
operations. At that point, `capacity` will be equal to two times `length`.
Future `push` operations will increase `length` until it reaches `capacity`,
at which point a relocation will happen and `capacity` will double again.

## `Vec::with_capacity`

To avoid having any relocations happen in the first place, you can use the
`Vec::with_capacity()` function to create the `Vec` with an initial capacity
that will be enough to hold all the elements you're planning to put in it.
(Assuming you know how many elements that will be, of course!)

This can prevent expensive relocations and also the wasted memory that can come
from `capacity` being doubled each time it runs out.
