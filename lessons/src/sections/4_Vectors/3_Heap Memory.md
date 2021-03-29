# Heap Memory

## Returning variable-length values on the stack

Let's say we want a function to return a `Vec`.

```rust
fn get_years() -> Vec<i32> {
    return vec![1995, 2000, 2005, 2010, 2015];
}

let years = get_years();
```

If all we have is stack memory, this would be very difficult.

Remember, the way we return values is by having the caller reserve space up 
front for the return value, which the called function can then write into 
before returning. However, in this case the caller can't know how much space
to reserve!

If the `Vec` being returned ends up being empty, the caller only needs to
reserve enough space for the `Vec`'s metadata struct. But it might not be
empty - it might be enormous! Should the caller reserve a gigabyte of space
just in case `get_years` happens to return an enormous `Vec`? That wouldn't
scale - if we did that, calling a couple of functions that return `Vec`s would
quickly run us out of stack space, leading to a Stack Overflow.

## The heap

Of course, returning `Vec` values is something we'll want to do on a regular
basis, so the approach of preallocating a huge amount of stack memory for them
"just in case" won't work. We need a way to return values whose size we can't possibly know at runtime. This is where the heap comes in.

The actual way this call to `get_years` works is that all it returns on the
stack is the `VecMetadata` struct we talked about previously:

```rust
struct VecMetadata {
    memory_index_of_first_element: usize,
    length: usize,
    capacity: usize,
}
```

Like all structs, this has a size that's hardcoded at compile-time. That means
the caller can reserve exactly enough space to make room for this struct, and
the function can return the `Vec` by writing its metadata into that space.
The actual elements of the `Vec` aren't stored in the `stack_bytes` array at 
all. Instead, those elements are stored in the `heap_bytes` array.
