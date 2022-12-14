# The Match Control Flow Construct

Rust has powerful control flow construct called ```match``` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
Pattern can be made up of literal values, variable names, wildcards, and many other things.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns that Bind to Values

Match arms can bind to the parts of the values that match the pattern.

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

We add a variable called ```state``` to the pattern that matches values of the variant ```Coin::Quarter```.
When a ```Coin::Quarter``` matches, the ```state``` variable will bind to the value of that quarter's state.
Then we can use ```state``` in the code arm.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

## Matching with ```Option<T>```

Instead of comparing coins, we'll compare the variants of ```Option<T>```, but the way that the ```match``` expression works remain the same.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

```match``` against an enum, bind a variable to the data inside, and then execute code based on it.

## Matches Are Exhaustive

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.

## Catch-all Patterns and the _ Placeholder

Catch-all pattern meets the requirement that ```match``` must be exhaustive.
We have to put the catch-all arm last because the patterns are evaluated in order.
```_``` is a special pattern that matches any value and does not bind to that value.
