# Defining and Instantiating Structs

Structs are just like tuples.
Unlike with tuples, in a struct you will name each piece of data so it's clear what the values mean.

## Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Creating Instance From Other Instances With Struct Update syntax

```rust
fn main() {
    // --snip--
    let user2 = User {
        email: String::from("someone.else@example.com"),
        ..user1
    };
}
```

We can no longer use ```user1``` after creating ```user2``` because the ```String``` in the ```username``` field of ```user1``` has been moved to ```user2```.

## Using Tuple Structs without named Fields to Create Different Types

Tuple structs have the added meaning the struct name provides but don't have names associated with their fields;
rather, they just have the types of the fields. You can use a ```.``` followed by the index to access a individual value.

## Unit-Like Structs Without Any Fields

Structs that don't have any fields are called unit-like structs because they behave similarly to (), the unit type.
