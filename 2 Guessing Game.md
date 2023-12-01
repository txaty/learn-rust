## Setting up

```bash
$ cargo new guessing_game
$ cd guessing_game
```

## Processing a Guess

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This is called the prelude, you have to bring that type into scope explicitly with a ```use``` statement.

In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.

To make a variable mutable, we add ```mut``` before the variable name:

```bash
let apple = 5; // immutable
let mut bananas = 5; // mutable
```

```read_line``` puts whatever the user enters in to the string we pass to it, but it also returns a ```Result``` value.
```Result``` is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
We call each possible state a variant.

```Result```'s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

## Generating a secret number

A crate is a collection of Rust source code files.

Filename: `Cargo.toml`

```toml
rand = "0.8.3"
```

Rust has a strong, static type system.
However, it also has type inference.

Rust allow us to shadow the previous value with a new one.
Shadowing lets us reuse the variable name rather than forcing us to create two unique variables.