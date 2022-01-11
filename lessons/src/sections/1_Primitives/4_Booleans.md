# Booleans

Rust also has a boolean value. A `bool` can be either `true` or `false`.
At runtime, `bool` is the same as `u8`, but at compile time they have different
types because they're used for different purposes.

> You can convert from `bool` to any of the number types using `as`. 
> `true` converts to `1` and `false` converts to `0`.

## The `==` operator

Rust's `==` operator returns `true` if two values are structurally equal,
and `false` if they are not.

```
1 == 2 // returns `false`
```

Rust has no `===` operator, just `==`.

## `if` conditionals

In Rust, `if` requires a `bool` condition and then some code inside a `{` and `}` block.

```
if cats > 1 {
    println!("Multiple cats!");
}
```

Rust does not have "truthiness" - so `if` must always be given a `bool`.
You also don't need to put parentheses around the condition, like you do in
many languages.

## `else` and `else if`

You can use `else` and `else if` like in most languages:

```
println!("My name is {}", name);

if same_name_as_parent_and_grandparent { 
    println!(" III");
} else if same_name_as_parent { 
    println!(" Jr");
} else if same_name_as_child { 
    println!(" Sr");
} else {
    println!("!");
};
```

## Using the result of `if` directly

You can use the result of an `if` directly as long as it includes an `else`
branch.

```
let suffix = if same_name_as_parent_and_grandparent { 
    "III" 
} else if same_name_as_parent { 
    "Jr"
} else if same_name_as_child { 
    "Sr" 
} else {
    "!"
};

println!("My name is {}{}", name, suffix);
```

Without the `else`, this would not compile, because Rust wouldn't know
what to set `suffix` to if all these conditions happened to be `false`. 
(Rust does not have an equivalent of JavaScript's `null` or `undefined`.)
