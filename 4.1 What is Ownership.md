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

## Memory and Allocation

In Rust, the memory is automatically returned once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
} // this scope is now over, and s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us.
This function is called ```drop```, and it's where the author of ```String``` can put the code to return the memory.
Rust calls ```drop``` automatically at the closing curly bracket.

### Ways Variables and Data Interact: Move

```rust
let s1 = String::from("hello");
let s2 = s1;
```

To ensure memory safety, after the line ```let s2 = s1;```, Rust considers ```s1``` to no longer be valid and, therefore, Rust doesn't need to free anything when ```s1``` goes out of scope.

In addition, there's a design choice that's implied by this: Rust will never automatically create "deep" copies of your data.
Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

### Ways Variables and Data Interact: Clone

If we do want to deeply copy the heap data of the ```String```, not just the stack data, we can use a common method called ```clone```.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

### Stack-Only Data: Copy

Rust has a special annotation called the ```Copy``` trait that we can place on types that are stored on the stack,
as integers are.
If a type implements the ```Copy``` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won't let us annotate a type with ```Copy``` if the type, or any of its parts, has implemented the ```Drop``` trait.
If the type needs something special to happen when the value goes out of scope and we add the ```Copy``` annotation to that type, we'll get a compile-time error.

As a general rule, any group of simple scalar values can implement ```Copy```, and nothing that requires allocation or is some form of resource can implement ```Copy```:

- All the integer types, such as ```u32```.
- The Boolean type, ```bool```, with values ```true``` and ```false```.
- All the floating point types, such as ```f64```.
- The character type, ```char```.
- Tuples, if they only contain types that also implement ```Copy```. For example, ```(i32, i32)``` implements ```Copy```, but ```(i32, String)``` does not.

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.
