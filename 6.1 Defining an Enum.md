# Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enum values

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

The variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
This is useful because now both values ```IpAddrKind::V4``` and ```IpAddrKind::V6``` are of the same type: ```IpAddrKind```.
We can then define a function that takes any ```IpAddrKind```:

```rust
fn route(ip_kind: IpAddrKind) {}
```

We can put data directly into each enum variant.

```rust
enum IpAddr {
    V4(string),
    V6(string),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

There's another advantage to using an enum rather that a struct: each variant can have different types and amounts of associated data.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

You can put any kind of data inside an enum variant: strings, numeric types, or structs.

We can define methods on enums.

```rust
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## The ```Option``` Enum and Its Advantage Over Null Values

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

```rust
enum Option<T> {
    None,
    Some(T),
}
```
