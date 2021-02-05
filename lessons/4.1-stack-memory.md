# Stack Memory

We need to talk about memory in order to understand some important Rust
concepts, but we're going to hand-wave away a lot of details. After all, this 
workshop is an introduction to Rust, not to operating systems and hardware!

## The Stack and the Heap

Practically all processes organize their giant memory array into two regions:
the "stack" and the "heap." The stack is specifically used to store data for
function arguments and return values.

So let's imagine that we have 4 gigabytes of memory, and it's subdivided into
1 gigabyte for the stack and 3 gigabytes for the heap. (The heap is almost always
much larger than the stack.)

```rust
let stack_bytes: [u8; 1_000_000_000];
let heap_bytes: [u8; 3_000_000_000];
```

> It's technically possible to resize these at runtime, but in practice it's 
> very rarely done.

# Adding arguments to the Stack

When we call a function, the arguments get written to stack memory. Consider
this function:

```rust
fn print_number(num: i8) {
    println!("Number: {}", num);
}

fn add_numbers(num1: i8, num2: i8) {
    print_number(num1 + num2);
}

add_numbers(5, 42);
```

When we run that last line of code, here's what happens:

1. Our program sets `stack_bytes[0] = 5` and `stack_bytes[1] = 42`
2. Our program also increments a special `stack_length` global variable twice, so it's at 2 now.
3. The `add_numbers` function starts running. It looks at `stack_bytes[stack_length - 2]` for its first argument, and `stack_bytes[stack_length - 1]` for its second argument.
4. It adds the two numbers and passes them to `print_number`. Passing arguments to `print_number` means writing the number to `stack_bytes[stack_length]` and then incrementing `stack_length`.
5. Now when `print_number` runs, it knows that its one argument is in `stack_bytes[stack_length - 1]`, just like `add_numbers` did.
6. When `print_number` returns, it decrements `stack_length`, meaning the bytes it was using for its argument are now effectively available for future function calls to use.
7. When `add_number` returns, it decrements `stack_length` twice, meaning it is now back down to 0 and the entire stack is free for future function calls.

Normally, this is how things go - as we nest function calls, `stack_length`
increases, and as they return, it decreases. If we have too many nested function
calls, `stack_length` might exceed the total number of bytes available in the
stack. This is called a Stack Overflow, and it normally crashes the program.

# Returning values on the Stack

Returning values on the stack works similarly to passing arguments.

```rust
fn add_numbers(num1: i8, num2: i8) -> i8 {
    return num1 + num2;
}

let answer = add_numbers(5, 42);
```

Here, the same operations happen as before for passing arguments. However,
before writing the arguments into `stack_bytes`, the caller increments 
`stack_length` an extra time up front, to leave a buffer for the return value.
This means that the arguments will be written one byte later than before, and
there will be a gap of unused memory right before them.

After `add_numbers` finishes running, it will decrement `stack_length` twice as 
usual (to free the stack space taken up by its arguments).
Now `stack_bytes[stack_length - 1]` will refer to that gap of unused memory we
created ahead of time. `add_numbers` will write the return value into that
slot before exiting the function.

Now when the caller resumes running, it knows that the return value from
`add_numbers` can be found in `stack_bytes[stack_length - 1]`. It also knows
that it needs to decrement `stack_length` one more time before it exits, to make
sure it frees up that stack memory for other functions.
