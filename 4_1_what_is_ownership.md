# What is Ownership

Ownership is a set of rules that governs how a Rust program manages memory.
Some languages have garbage collection that regularly looks for no-longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
If any of the rules are violated, the program won't compile.

## Stack and Heap

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data;
that location is always at the top of the stack.
Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.

## Ownership Rules

The ownership rules are:

- Each value in Rust has an owner,
- There can only be one owner at a time,
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

```rust
{   // s is not valid here, it's not yet declared
    let s = "hello";    // s is valid from this point forward
}   // this scope is now over, and s is no longer valid
```

## The String Type

Rust has a second string type, ```String```.
This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
You can create a ```String``` from a string literal using the ```from``` function:

```rust
let s = String::from("hello");
```

This kind of string can be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```



