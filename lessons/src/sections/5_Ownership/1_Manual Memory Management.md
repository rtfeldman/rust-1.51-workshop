## `alloc` and `dealloc`

Compared to the stack, the heap is more versatile but more work to manage.

On the stack, all bytes from `stack_length` onward are available for use. All
the bytes before it are reserved for use in functions that are currently
executing. That's all there is to it.

In contrast, heap memory is decoupled from function execution. Here's how it
works:

* When a function like `Vec`'s `.push()` method wants some heap memory, Rust's `alloc` function must be called, passing the number of bytes needed to store the `Vec`'s elements.
* `alloc` looks at Rust's global allocation bookkeeping system, which tracks which bytes are in use, and finds a contiguous chunk of bytes which is at least as long as the requested number of bytes. (Such a chunk can take awhile to find; `alloc` is a common performance bottleneck!)
* When that `Vec` is no longer needed, `dealloc` must be called to inform the bookkeeping system that those bytes are no longer in use. If this doesn't happen, we have a "memory leak" - which will cause the program to use more system memory than it actually needs.

## Manual memory management

In C, memory management functions like `alloc` and `dealloc` must be called 
explicitly by the programmer. Calling `alloc` is relatively straightforward, 
because it happens right when you actually need the bytes. However, figuring 
out when to call `dealloc` is notoriously error-prone.

Let's go back to our example from earlier, and add a couple of `println!` calls.

```rust
fn get_years() -> Vec<i32> {
    return vec![1995, 2000, 2005, 2010, 2015];
}

let years = get_years();

println!("The highest year is {}", find_highest_year(years));
println!("All done!");
```

If we were doing manual memory management, `get_years` would actually call
`alloc` to reserve enough space for five `i32` values on the heap. It would
then return a `VecMetadata` struct whose `memory_index_of_first_element` value
would be an index into the `heap_bytes` array. (We could have called it
`heap_bytes_index_of_first_element` to be extra clear about this.)

However, when should `dealloc` be called?

One approach would be to call it at the very end of the program. Using that as a
general strategy - deallocating everything at the very end of the program - is
straightforward, but means that while the program is running, its heap memory 
usage only ever grows and never decreases. This is essentially the same thing
as leaking all heap memory on purpose, which makes it about the same as never
bothering to call `dealloc` at all!

Another approach would be to `dealloc` the heap memory as soon as it's no longer
in use. In this example, that would be right between the two `println!` calls.
This is optimal from a memory usage perspective; we are using the memory only
as long as we need to.

## Potential Mistakes

Let's say we make a mistake, though. What if we `dealloc` the memory before
either `println!` call? That means the `find_highest_year` function will be
reading heap memory that has potentially been overwritten with other data...and
that other data could be anything by now! Instead of years, we could be looking
at bytes representing recipes in a cookbook.

This is known as a "use-after-free" bug, and it happens when memory is accessed
after it has been "freed" (deallocated). These can be extremely painful to 
debug, because the symptoms are that sometimes some of your data is random
gibberish instead of what you expect.

Another potential mistake is that we could call `dealloc` twice on the same 
memory index. This is called a "double free," and it can lead to the same
symptoms as "use-after-free" bugs. This is because once memory has been freed, 
the global allocation bookkeeping system may hand that "unused" memory index 
out to a later `alloc` request. If we then call `dealloc` a second time on
that memory index, it will be deallocating memory that is still in use (by
the newer allocation), which is a use-after-free!
